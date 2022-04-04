//RUST_LOG=debug;
#[cfg(test)]
mod tests {
    use log::{debug, error, info, trace, warn};
    #[test]
    fn test() {
        env_logger::init();
        info!("Hello, world!");
        trace!("Hello, world!");
        warn!("Hello, world!");
        debug!("Hello, world!");
        error!("Hello, world!");
    }
}
