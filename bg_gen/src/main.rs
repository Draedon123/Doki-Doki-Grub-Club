use std::{
  env,
  error::Error,
  io::{self, Read, Write},
  process,
};

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|error| {
    eprintln!("Error while parsing arguments: {error}");
    process::exit(1);
  });

  println!("{}x{}px", config.width, config.height);
}

struct Config {
  width: u16,
  height: u16,
}

impl Config {
  fn new(args: &[String]) -> Result<Self, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }

    let width: u16 = u16::from_str_radix(&args[1], 10).unwrap_or_else(|_| {
      eprintln!("Invalid width");
      pause();
      process::exit(1);
    });
    let height: u16 = u16::from_str_radix(&args[2], 10).unwrap_or_else(|_| {
      eprintln!("Invalid height");
      pause();
      process::exit(1);
    });

    Ok(Self { width, height })
  }
}

// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/4
fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press any key to exit").unwrap();
  stdout.flush().unwrap();

  let _ = stdin.read(&mut [0u8]).unwrap();
}
