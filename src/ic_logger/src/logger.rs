extern crate log;

use ic_cdk::{println, eprintln};
use log::{Record, Level, Metadata,
          SetLoggerError, LevelFilter};

include!(concat!(env!("OUT_DIR"), "/env.rs"));

struct ICLogger;

impl log::Log for ICLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
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
        "error" => {
            ic_cdk::println!("Setting log level to: {:?}", CONFIG.logger);
            LevelFilter::Error
        },
        "warn" => {
            ic_cdk::println!("Setting log level to: {:?}", CONFIG.logger);
            LevelFilter::Warn
        },
        "info" => {
            ic_cdk::println!("Setting log level to: {:?}", CONFIG.logger);
            LevelFilter::Info
        },
        "debug" => {
            ic_cdk::println!("Setting log level to: {:?}", CONFIG.logger);
            LevelFilter::Debug
        },
        "trace" => {
            ic_cdk::println!("Setting log level to: {:?}", CONFIG.logger);
            LevelFilter::Trace
        },
        _ => {
            ic_cdk::println!("Setting log level to: {:?}", CONFIG.logger);
            LevelFilter::Error
        },
    }
}





