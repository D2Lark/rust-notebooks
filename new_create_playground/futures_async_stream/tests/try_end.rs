#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use futures::stream::Stream;
use futures_async_stream::{for_await, stream};
use tokio::time::{sleep, Duration};
use futures::StreamExt;
#[tokio::test]
async fn i32_test() {


}


#[stream(boxed,item = u64)]
async fn foo()  {
    yield 3;
    yield 4;
    yield 5;
}
