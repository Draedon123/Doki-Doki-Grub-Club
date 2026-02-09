use crate::config::Config;

pub mod config;
pub mod pause;

pub fn run(config: &Config) {
  println!("{}x{}px", config.width, config.height);
}
