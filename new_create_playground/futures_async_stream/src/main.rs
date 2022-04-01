#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use futures::{
    future::Future,
    pin_mut,
    stream::Stream,
    task::{noop_waker, Context, Poll},
};
use tokio::time::{Duration,sleep};
use futures_async_stream::{for_await, stream, stream_block};

#[stream(item = u64)]
async fn stream1() {
    yield 0;
    yield 1;
}

#[tokio::main]
async fn main(){
    let mut v = 0..=1;
    #[for_await]
    for x in bar(foo()) {

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