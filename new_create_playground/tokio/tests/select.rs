use tokio::time::{sleep, Duration};
use tokio::{select, signal};
use tokio_util::sync::CancellationToken;
#[tokio::test]
async fn test_select() {
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    let handle = tokio::spawn({
        async move {
            loop {
                sleep_1s().await;
            }
        }
    });
    let handle = tokio::spawn(async move {
        select! {
            _  = signal::ctrl_c() => {
                cloned_token.cancel();
                5
            },
            ret = handle => {
                println!("handle ret: {:?}", ret);
                10
            }
        }
    });
    println!("handle: {:?}", handle.await.unwrap());
}

async fn sleep_1s() {
    println!("sleep_1s");
    sleep(Duration::from_secs(1)).await;
}
