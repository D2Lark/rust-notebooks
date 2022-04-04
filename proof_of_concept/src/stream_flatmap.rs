#[cfg(test)]
mod tests {

    use futures::stream;
    use futures::StreamExt;
    use futures_async_stream::stream;
    #[stream(boxed,item=i32)]
    async fn stream2() {
        for i in 1..4 {
            yield i;
        }
    }
    #[tokio::test]
    async fn test() {
        let stream = stream2();
        let mut stream = stream.flat_map(|x| stream::iter(vec![x + 3; x as usize]));

        println!("i:{:?}", stream.next().await);
        println!("i:{:?}", stream.next().await);
        println!("i:{:?}", stream.next().await);
        println!("i:{:?}", stream.next().await);
        println!("i:{:?}", stream.next().await);
        println!("i:{:?}", stream.next().await);
    }
}
