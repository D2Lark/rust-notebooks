#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use anyhow::Result;
use futures::{stream::Stream, StreamExt};
use futures_async_stream::{for_await, stream};
use minitrace::prelude::*;
use tokio::time::{sleep, Duration};
#[tokio::test]
async fn i32_test() {
    todo!();
}
