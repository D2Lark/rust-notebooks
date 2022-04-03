#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
use anyhow::Result;
use futures::stream::Stream;
use futures_async_stream::{for_await, stream};
use std::fs::File;
use tokio::sync::mpsc::{self, Sender};
#[tokio::test]
async fn test_channel() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<Vec<String>>(1);
    read_file_blocking(tx).await?;
    // while let Some(data) = rx.recv().await
    // {
    //     println!("{:?}", data);
    // }
    Ok(())
}

async fn read_file_blocking(tx: Sender<Vec<String>>) -> Result<()> {
    let file = File::open("testdata/employee.csv")?;
    println!("read_file_blocking");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let mut iter = reader.records();
    while let Some(record) = iter.next() {
        let record = record?;
        for str in &record {
            println!("{}", str);
        }
    }
    Ok(())
}
