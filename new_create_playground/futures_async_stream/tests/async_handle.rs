#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use futures::stream::{self, Stream};
use futures::StreamExt;
use futures_async_stream::{for_await, stream};
use tokio::pin;
use tokio::time::{sleep, Duration};
#[tokio::test]
async fn i32_test() {
    // #[for_await]
    // for x in bar(foo()) {
    //     println!("resutl is {}", x);
    // }\
    let stream = bar(foo());
    pin!(stream);
    while let Some(x) = stream.next().await {
        println!("resutl is {}", x);
    }
}

fn foo() -> impl Stream<Item = i32> {
    #[stream]
    async move {
        for i in 0..10 {
            println!("sleep 1 s");
            sleep(Duration::from_secs(1)).await;
            yield i;
        }
    }
}

#[stream(item = i32)]
async fn bar(stream: impl Stream<Item = i32>) {
    // `for_await` is built into `stream`. If you use `for_await` only in `stream`, there is no need to import `for_await`.
    #[for_await]
    for x in stream {
        println!("processing {}", x);
        sleep(Duration::from_secs(1)).await;
        yield x + 2;
    }
}
