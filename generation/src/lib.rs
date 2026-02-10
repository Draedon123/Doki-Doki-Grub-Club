use crate::config::Config;

pub mod background;
pub mod config;
pub mod constants;
pub mod pause;

pub fn run(config: &Config) {
  background::generate_background(config.width, config.height).unwrap_or_else(|error| {
    eprintln!("{:#?}", error);
  });
}
