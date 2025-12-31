use crate::config::{Config, LogLevel};
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

pub fn init_logger(config: &Config) {
  let log_level = match config.log_level {
    LogLevel::Trace => LevelFilter::Trace,
    LogLevel::Debug => LevelFilter::Debug,
    LogLevel::Info => LevelFilter::Info,
    LogLevel::Warn => LevelFilter::Warn,
    LogLevel::Error => LevelFilter::Error,
  };

  let env = config.environment.clone();
  let service_name = config.name.clone();

  Builder::new()
    .target(Target::Stdout)
    .filter_level(log_level)
    .format(move |buf, record| {
      writeln!(
        buf,
        "[{} {} {}:{}] env={} service={} pid={} lang=Rust | {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        record.file().unwrap_or("unknown"),
        record.line().unwrap_or(0),
        env,
        service_name,
        std::process::id(),
        record.args()
      )
    })
    .init();

  log::info!("Logger initialized with level: {}", config.log_level);
  log::info!("Environment: {:?}", config.environment);
}
