use log::{Record, Level, Metadata};
use chrono::{DateTime, Utc};
use log::{SetLoggerError, LevelFilter};

pub struct MainLogger;

impl log::Log for MainLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Info
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			let now: DateTime<Utc> = Utc::now();
			println!("[{}][{}] {}", now.format("%Y-%m-%d %H:%M:%S"), record.level(), record.args());
		}
	}

	fn flush(&self) {}
}


static LOGGER: MainLogger = MainLogger;

pub fn init_log() -> Result<(), SetLoggerError> {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LevelFilter::Info))
}