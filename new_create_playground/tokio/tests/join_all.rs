use tokio::time::{Duration,sleep};
use futures::future::join_all;
#[tokio::test]
async fn test_spawn(){
    let handles = (1..3).map(|n|
        sleep_n_s(n)
    ).collect::<Vec<_>>();
    join_all(handles).await;

}

async fn sleep_n_s(n: u64) {
    loop{
        println!("sleep_{}_s", n);
        sleep(Duration::from_secs(n)).await;
    }

}

