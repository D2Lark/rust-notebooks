use futures::future::join_all;
use tokio::time::{sleep, Duration};
#[tokio::test]
async fn test_spawn() {
    let handles = (1..3).map(sleep_n_s).collect::<Vec<_>>();
    join_all(handles).await;
}

async fn sleep_n_s(n: u64) {
    loop {
        println!("sleep_{}_s", n);
        sleep(Duration::from_secs(n)).await;
    }
}
