use std::error::Error;

use image::{
  DynamicImage, GenericImageView, ImageReader,
  imageops::{FilterType, overlay},
};

pub fn generate_sidebar(
  canvas: &mut DynamicImage,
  width: u32,
  height: u32,
) -> Result<(), Box<dyn Error>> {
  let sidebar = ImageReader::open("./bg_gen/assets/main_menu.png")?
    .with_guessed_format()?
    .decode()?;

  let sidebar_dimensions = sidebar.dimensions();
  let sidebar_dimensions = (sidebar_dimensions.0 as f32, sidebar_dimensions.1 as f32);
  let sidebar_scale: f32 = height as f32 / sidebar_dimensions.1;
  let sidebar_dimensions = (
    (sidebar_dimensions.0 * sidebar_scale) as u32,
    (sidebar_dimensions.1 * sidebar_scale) as u32,
  );

  let sidebar = sidebar
    .resize(
      sidebar_dimensions.0,
      sidebar_dimensions.1,
      FilterType::Lanczos3,
    )
    .crop(0, 0, width, height);

  overlay(canvas, &sidebar, 0, 0);

  Ok(())
}
