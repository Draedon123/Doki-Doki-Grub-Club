use crate::config::Config;

pub mod config;
pub mod constants;
pub mod generate_background;
pub mod generation;
pub mod pause;

pub fn run(config: &Config) {
  generate_background::generate_background(config.width, config.height).unwrap_or_else(|error| {
    eprintln!("{:#?}", error);
  });
}
