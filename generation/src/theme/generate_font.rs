use std::{error::Error, process::Command};

pub fn generate_font(size: u32) -> Result<(), Box<dyn Error>> {
  let output = Command::new("sh").arg("-c").arg(format!(
    "grub-mkfont -o ./ddgc/RifficFree-Bold-{size}.pf2 -s {size} ./generation/assets/RifficFree-Bold.ttf"
  )).output()?;

  eprintln!("{}", String::from_utf8(output.stderr)?);

  Ok(())
}
