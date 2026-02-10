use crate::constants::{INTERNAL_LOGO_POSITION, INTERNAL_LOGO_SCALE, INTERNAL_SIZE};
use image::{
  DynamicImage, GenericImageView, ImageReader,
  imageops::{FilterType, overlay},
};
use std::error::Error;

pub fn generate_logo(
  canvas: &mut DynamicImage,
  _width: u32,
  height: u32,
) -> Result<(), Box<dyn Error>> {
  let logo = ImageReader::open("./generation/assets/logo.png")?
    .with_guessed_format()?
    .decode()?;

  let logo_dimensions = logo.dimensions();
  let logo_dimensions = (logo_dimensions.0 as f32, logo_dimensions.1 as f32);
  let logo_scale = height as f32 / INTERNAL_SIZE.1 as f32;
  let logo_dimensions = (
    (logo_dimensions.0 * logo_scale * INTERNAL_LOGO_SCALE) as u32,
    (logo_dimensions.1 * logo_scale * INTERNAL_LOGO_SCALE) as u32,
  );

  let logo_position = (
    (INTERNAL_LOGO_POSITION.0 as f32 * logo_scale - 0.5 * (logo_dimensions.0 as f32)) as i64,
    (INTERNAL_LOGO_POSITION.1 as f32 * logo_scale - 0.5 * (logo_dimensions.1 as f32)) as i64,
  );

  let logo = logo.resize(logo_dimensions.0, logo_dimensions.1, FilterType::Lanczos3);

  overlay(canvas, &logo, logo_position.0, logo_position.1);

  Ok(())
}
