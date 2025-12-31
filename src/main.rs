mod config;
mod adapters;

use config::Config;
use crate::adapters::log;

fn main() {
  let config = Config::from_env();
  log::init_logger(&config);

  println!("Config is ready, {:?}", config);
}
