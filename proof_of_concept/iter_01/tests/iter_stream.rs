#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]



use futures::stream::{self, Stream};
use futures::{pin_mut, StreamExt};
use futures_async_stream::{for_await, stream};

#[tokio::test]
async fn test() {
    let mut stream = stream2();

    println!("i:{:?}",stream.next().await);
    println!("i:{:?}",stream.next().await);
    println!("i:{:?}",stream.next().await);
    println!("i:{:?}",stream.next().await);
    
}

#[stream(boxed,item=String)]
async fn stream2() {
    for i in 1..4 {
        yield "abc".to_string();
    }
}
