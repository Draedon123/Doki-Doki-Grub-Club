use image::{self, DynamicImage};
use std::error::Error;

use crate::generation::{
  backdrop::generate_backdrop, logo::generate_logo, sidebar::generate_sidebar,
};

pub fn generate_background(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
  let mut canvas = DynamicImage::new_rgba8(width, height);

  generate_backdrop(&mut canvas, width, height)?;
  generate_sidebar(&mut canvas, width, height)?;
  generate_logo(&mut canvas, width, height)?;

  canvas.save("./background.png")?;

  Ok(())
}
