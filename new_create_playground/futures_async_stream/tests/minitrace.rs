#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use futures::stream::Stream;
use futures::StreamExt;
use futures_async_stream::{for_await, stream};
use minitrace::prelude::*;
use tokio::pin;
use tokio::time::{sleep, Duration};
#[tokio::test]
async fn i32_test() {
    let (root, collector) = Span::root("root");
    run().in_span(root).await;
    let records: Vec<SpanRecord> = collector.collect().await;
    let bytes = minitrace_jaeger::encode("asynchronous".to_owned(), 0, 0, 0, &records).unwrap();
    minitrace_jaeger::report("127.0.0.1:6831".parse().unwrap(), &bytes)
        .await
        .ok();
}
async fn run() {
    let stream = foo(bar(source()));
    pin!(stream);
    while let Some(x) = stream.next().await {
        sleep(Duration::from_secs(1)).await;
        println!("resutl is {}", x);
    }
}
#[trace("other job")]
fn source() -> impl Stream<Item = i32> {
    #[stream]
    async move {
        for i in 0..3 {
            sleep(Duration::from_secs(1)).await;
            yield i;
        }
    }
}

#[stream(item = i32)]
async fn bar(stream: impl Stream<Item = i32>) {
    // `for_await` is built into `stream`. If you use `for_await` only in `stream`, there is no need to import `for_await`.
    pin!(stream);
    while let Some(x) = stream
        .next()
        .await
    {
        println!("bar span {}", x);
        sleep(Duration::from_secs(2)).await;
        yield x + 2;
    }
}

#[stream(item = i32)]
async fn foo(stream: impl Stream<Item = i32>) {
    // `for_await` is built into `stream`. If you use `for_await` only in `stream`, there is no need to import `for_await`.
    pin!(stream);
    while let Some(x) = stream
        .next()
        .await
    {
        println!("foo span {}", x);
        sleep(Duration::from_secs(2)).await;
        yield x - 2;
    }
}
