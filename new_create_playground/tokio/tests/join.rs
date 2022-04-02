use tokio::time::{Duration,sleep};
use futures::join;
#[tokio::test]
async fn test_spawn(){
join!(sleep_n_s(4),sleep_n_s(2));

}

async fn sleep_n_s(n: u64) {
    loop{
        println!("sleep_{}_s", n);
        sleep(Duration::from_secs(n)).await;
    }

}

