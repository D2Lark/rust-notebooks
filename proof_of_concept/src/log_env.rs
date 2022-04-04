use log::{debug, error, info, trace, warn};
//RUST_LOG=debug;
fn main() {
    env_logger::init();
    info!("Hello, world!");
    trace!("Hello, world!");
    warn!("Hello, world!");
    debug!("Hello, world!");
    error!("Hello, world!");
}
