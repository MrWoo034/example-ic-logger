use log::{debug, error, info, trace, warn};
use ic_cdk_macros::init;

mod logger;

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[init]
fn init() {
    logger::init();
    trace!("Trace statement for logging message number: {}", 1);
    debug!("Debug statement for logging message number: {}", 2);
    info!("Info statement for logging message number: {}", 3);
    warn!("Warn statement for logging message number: {}", 4);
    error!("Error statement for logging message number: {}", 5);
}
