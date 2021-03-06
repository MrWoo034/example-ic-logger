extern crate log;

use ic_cdk::{eprintln, println};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

include!(concat!(env!("OUT_DIR"), "/env.rs"));

struct ICLogger;

impl log::Log for ICLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= get_log_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if record.level() == Level::Error {
                eprintln!("[{}] - {}", record.level(), record.args())
            } else {
                println!("[{}] - {}", record.level(), record.args());
            }
        }
    }

    fn flush(&self) {
        todo!()
    }
}

static LOGGER: ICLogger = ICLogger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(get_log_level());
}

fn get_log_level() -> LevelFilter {
    match CONFIG.logger {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Error,
    }
}
