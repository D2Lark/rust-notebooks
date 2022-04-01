use clap::Parser;
use minitrace::prelude::*;
use tokio::time::{sleep, Duration};
#[derive(Parser, Debug)]
struct Args {
    /// Whether to enable tracing
    #[clap(long, short)]
    enable_tracing: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let enable_tracing = args.enable_tracing;
    if enable_tracing {
        let (root, collector) = Span::root("root");
        async {
            sql_run()
                .in_span(Span::enter_with_local_parent("sql running"))
                .await;
            parsing()
                .in_span(Span::enter_with_local_parent("parsing running"))
                .await;
            execution()
                .in_span(Span::enter_with_local_parent("execution  running"))
                .await;
        }
        .in_span(root)
        .await;

        let records: Vec<SpanRecord> = collector.collect().await;
        println!("records:{records:?}");
    } else {
        sql_run().await;
    }
}

async fn sql_run() {
    println!("SQL runing");
    sleep(Duration::from_secs(3)).await;
    println!("SQL Done");
}

async fn parsing() {
    println!("parsing Running");
    sleep(Duration::from_secs(3)).await;
    println!("parsing Done");
}

async fn execution() {
    println!("execution Running");
    sleep(Duration::from_secs(3)).await;
    println!("execution Done");
}
