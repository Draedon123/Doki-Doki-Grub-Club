use crate::constants::INTERNAL_SIZE;
use image::{
  DynamicImage, GenericImageView, ImageReader,
  imageops::{FilterType, overlay},
};
use std::error::Error;

pub fn generate_character(
  canvas: &mut DynamicImage,
  _width: u32,
  height: u32,
  sprite_path: &str,
  x: u32,
  y: u32,
  scale: f32,
) -> Result<(), Box<dyn Error>> {
  let sprite = ImageReader::open(sprite_path)?
    .with_guessed_format()?
    .decode()?;

  let sprite_dimensions = sprite.dimensions();
  let sprite_dimensions = (sprite_dimensions.0 as f32, sprite_dimensions.1 as f32);
  let sprite_scale = height as f32 / INTERNAL_SIZE.1 as f32;
  let sprite_dimensions = (
    (sprite_dimensions.0 * sprite_scale * scale) as u32,
    (sprite_dimensions.1 * sprite_scale * scale) as u32,
  );

  let sprite_position = (
    (x as f32 * sprite_scale - 0.5 * (sprite_dimensions.0 as f32)) as i64,
    (y as f32 * sprite_scale - 0.5 * (sprite_dimensions.1 as f32)) as i64,
  );

  let sprite = sprite.resize(
    sprite_dimensions.0,
    sprite_dimensions.1,
    FilterType::Lanczos3,
  );

  overlay(canvas, &sprite, sprite_position.0, sprite_position.1);

  Ok(())
}
