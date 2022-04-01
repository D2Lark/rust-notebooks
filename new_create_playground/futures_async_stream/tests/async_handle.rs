#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use futures::{
    stream::Stream,

};
use futures_async_stream::{for_await, stream, stream_block};
use tokio::time::{Duration,sleep};
#[tokio::test]
async fn handle_test(){

    #[for_await]
    for x in bar(foo()) {

    }
}


fn foo() -> impl Stream<Item = ()> {
    #[stream]
    async move {
        for i in 0..10 {
            println!("foo:sleep 1 s");
            sleep(Duration::from_secs(1)).await;
            yield ();
        }
    }
}
#[stream(item = ())]
async fn bar(stream: impl Stream<Item = ()>) {
    // `for_await` is built into `stream`. If you use `for_await` only in `stream`, there is no need to import `for_await`.
    #[for_await]
    for x in stream {
        println!("processing bar ");
        sleep(Duration::from_secs(1)).await;
        yield ();
    }
}