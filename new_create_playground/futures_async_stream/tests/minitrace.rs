#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use futures::stream::BoxStream;
use futures_async_stream::{for_await, stream};
use tokio::time::{sleep, Duration};
#[tokio::test]
async fn i32_test() {
    #[for_await]
    for x in stream3(stream2(stream1())) {
        println!("resutl is {}", x);
    }
}
pub type BoxedExecutor = BoxStream<'static, i32>;

#[stream(item = i32, boxed)]
pub async fn stream1() {
    for i in 1..=3 {
        println!("stream 1=>init:{}", i);
        sleep(Duration::from_secs(1)).await;
        yield i;
    }
}

#[stream(boxed, item = i32)]
async fn stream2(executor_stream: BoxedExecutor) {
    #[for_await]
    for x in executor_stream {
        println!("stream 2=>2 + {}", x);
        sleep(Duration::from_secs(1)).await;
        yield x + 2;
    }
}

#[stream(boxed, item = i32)]
async fn stream3(executor_stream: BoxedExecutor) {
    #[for_await]
    for x in executor_stream {
        println!("stream 3=>3 + {}", x);
        sleep(Duration::from_secs(1)).await;
        yield x + 3;
    }
}
