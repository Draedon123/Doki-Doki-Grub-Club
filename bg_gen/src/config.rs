use std::process;

use crate::pause::pause;

pub struct Config {
  pub width: u32,
  pub height: u32,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }

    let width: u32 = u32::from_str_radix(&args[1], 10).unwrap_or_else(|_| {
      eprintln!("Invalid width");
      pause();
      process::exit(1);
    });
    let height: u32 = u32::from_str_radix(&args[2], 10).unwrap_or_else(|_| {
      eprintln!("Invalid height");
      pause();
      process::exit(1);
    });

    Ok(Self { width, height })
  }
}
