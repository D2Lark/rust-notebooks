#[cfg(test)]
mod test {
    use futures::StreamExt;
    use futures_async_stream::{for_await, try_stream};
    #[tokio::test]
    async fn test() {
        let stream = stream1().flat_map(|x| stream2(x));
        #[for_await]
        for char in stream {
            println!("{:?}", char);
        }
    }

    #[try_stream(boxed,ok = String, error = i32)]
    async fn stream1() {
        println!("yield hello");
        yield String::from("hello");
        println!("yield world");
        yield String::from("world");
        yield Err(1)?;
    }
    #[try_stream(boxed,ok = char, error = i32)]
    async fn stream2(result: Result<String, i32>) {
        match result {
            Ok(string) => {
                for char in string.chars() {
                    yield char
                }
            }
            Err(i) => yield Err(i)?,
        }
    }
}
