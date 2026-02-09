use image::{
  self, DynamicImage, GenericImage, GenericImageView, ImageReader,
  imageops::{FilterType, overlay},
};
use std::error::Error;

pub fn generate_background(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
  let mut canvas = DynamicImage::new_rgba8(width, height);
  let background = ImageReader::open("./assets/menu_bg.png")?
    .with_guessed_format()?
    .decode()?;
  let sidebar = ImageReader::open("./assets/main_menu.png")?
    .with_guessed_format()?
    .decode()?;
  // let logo = ImageReader::open("./assets/logo.png")?
  //   .with_guessed_format()?
  //   .decode()?;

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

  canvas.copy_from(&background, 0, 0)?;
  // canvas.copy_from(&sidebar, 0, 0)?;
  overlay(&mut canvas, &sidebar, 0, 0);

  canvas.save("./background.png")?;

  Ok(())
}
