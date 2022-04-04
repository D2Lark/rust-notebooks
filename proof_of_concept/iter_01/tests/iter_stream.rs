#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use futures_async_stream::{for_await, stream, try_stream, try_stream_block};
use futures::stream::{self, Stream};
use futures::{pin_mut, StreamExt};

#[tokio::test]
async fn test() {
    let stream = stream1().flat_map(|x| stream2(x));
    #[for_await]
    for char in stream{
        println!("{:?}", char);
    }
}

#[try_stream(boxed,ok = String, error = i32)]
async fn stream1() {
    println!("yield hello");
    yield String::from("hello");
    println!("yield world");
    yield String::from("world");
    yield Err(1)?;

}
#[try_stream(boxed,ok = char, error = i32)]
async fn stream2(result :Result<String,i32>) {
    match result{
        Ok(string) => {for char in string.chars() {yield char}},
        Err(i) =>{ yield Err(i)? }
    }
}
