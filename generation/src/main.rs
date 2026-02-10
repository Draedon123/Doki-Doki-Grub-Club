use generation::{config::Config, pause::pause, run};
use std::{env, process};

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|error| {
    eprintln!("Error while parsing arguments: {error}");
    pause();
    process::exit(1);
  });

  run(&config)
}
