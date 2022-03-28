use log::{info, trace, warn,debug,error};
fn main() {
    env_logger::init();
    info!("Hello, world!");
    trace!("Hello, world!");
    warn!("Hello, world!");
    debug!("Hello, world!");
    error!("Hello, world!");
}
