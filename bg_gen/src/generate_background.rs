use image::{self, DynamicImage};
use std::error::Error;

use crate::{
  constants::*,
  generation::{
    backdrop::generate_backdrop, character::generate_character, logo::generate_logo,
    sidebar::generate_sidebar,
  },
};

pub fn generate_background(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
  let mut canvas = DynamicImage::new_rgba8(width, height);

  generate_backdrop(&mut canvas, width, height)?;
  generate_sidebar(&mut canvas, width, height)?;
  generate_logo(&mut canvas, width, height)?;

  generate_character(
    &mut canvas,
    width,
    height,
    "./bg_gen/assets/menu_art_y.png",
    YURI_POSITION.0,
    YURI_POSITION.1,
    YURI_ZOOM,
  )?;
  generate_character(
    &mut canvas,
    width,
    height,
    "./bg_gen/assets/menu_art_n.png",
    NATSUKI_POSITION.0,
    NATSUKI_POSITION.1,
    NATSUKI_ZOOM,
  )?;
  generate_character(
    &mut canvas,
    width,
    height,
    "./bg_gen/assets/menu_art_s.png",
    SAYORI_POSITION.0,
    SAYORI_POSITION.1,
    SAYORI_ZOOM,
  )?;
  generate_character(
    &mut canvas,
    width,
    height,
    "./bg_gen/assets/menu_art_m.png",
    MONIKA_POSITION.0,
    MONIKA_POSITION.1,
    MONIKA_ZOOM,
  )?;

  canvas.save("./background.png")?;

  Ok(())
}
