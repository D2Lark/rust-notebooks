
#![feature(generators)]
use futures::{stream::{self,Stream}, StreamExt};
use futures_async_stream::stream;
fn test_1(){
    let vec = stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    let v =  foo(vec);
    assert_eq!(v.poll_next_unpin(cx), Poll::Ready(1));

}


#[stream(item = i32)]
async fn foo(stream: impl Stream<Item = String>) {
    // `for_await` is built into `stream`. If you use `for_await` only in `stream`, there is no need to import `for_await`.
    #[for_await]
    for x in stream {
        yield x.parse().unwrap();
    }
}