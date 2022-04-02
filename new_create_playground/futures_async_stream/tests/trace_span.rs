#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use futures::{stream::Stream, StreamExt};
use futures_async_stream::{for_await, stream};
use tokio::time::{sleep, Duration};
use anyhow::Result;
use minitrace::prelude::*;
#[tokio::test]
async fn i32_test() {
    todo!();
}
