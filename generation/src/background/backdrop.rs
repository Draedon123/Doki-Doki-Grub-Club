use std::error::Error;

use image::{
  DynamicImage, GenericImageView, ImageReader,
  imageops::{FilterType, overlay},
};

pub fn generate_backdrop(
  canvas: &mut DynamicImage,
  width: u32,
  height: u32,
) -> Result<(), Box<dyn Error>> {
  let background = ImageReader::open("./generation/assets/menu_bg.png")?
    .with_guessed_format()?
    .decode()?;

  let background_dimensions = background.dimensions();
  let background_dimensions = (
    background_dimensions.0 as f32,
    background_dimensions.1 as f32,
  );
  let background_scale: f32 = ((width as f32) / background_dimensions.0 as f32)
    .max((height as f32) / background_dimensions.1 as f32);
  let background_dimensions = (
    (background_dimensions.0 as f32 * background_scale) as u32,
    (background_dimensions.1 as f32 * background_scale) as u32,
  );

  let background = background
    .resize(
      background_dimensions.0,
      background_dimensions.1,
      FilterType::Lanczos3,
    )
    .crop(0, 0, width, height);

  overlay(canvas, &background, 0, 0);

  Ok(())
}
