use std::io::{self, Read, Write};

// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/4
pub fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press any key to exit").unwrap();
  stdout.flush().unwrap();

  let _ = stdin.read(&mut [0u8]).unwrap();
}
