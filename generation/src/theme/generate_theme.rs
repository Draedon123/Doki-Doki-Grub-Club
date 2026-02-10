use std::{error::Error, fs};

use crate::{constants::*, theme::generate_font::generate_font};

pub fn generate_theme(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
  let theme = fs::read_to_string("./generation/assets/theme_template.txt")?;
  let scale = (
    width as f32 / INTERNAL_SIZE.0 as f32,
    height as f32 / INTERNAL_SIZE.1 as f32,
  );

  let navigation_x: u32 = (NAVIGATION_X as f32 * scale.0) as u32;
  let navigation_spacing: u32 = (NAVIGATION_SPACING as f32 * scale.1) as u32;
  let main_font_size: u32 =
    round_to_nearest_multiple((INTERFACE_FONT_SIZE as f32 * scale.1) as u32, 4);
  let navigation_outline = (NAVIGATION_OUTLINE as f32 * scale.1) as u32;

  generate_font(main_font_size)?;
  generate_font(TERMINAL_FONT_SIZE)?;

  let theme = theme.replace("__NAVIGATION_X__", &navigation_x.to_string());
  let theme = theme.replace("__NAVIGATION_SPACING__", &navigation_spacing.to_string());
  let theme = theme.replace("__NAVIGATION_OUTLINE__", &navigation_outline.to_string());
  let theme = theme.replace(
    "__NAVIGATION_OUTLINE_LEFT_X__",
    &(navigation_x - navigation_outline).to_string(),
  );
  let theme = theme.replace(
    "__NAVIGATION_OUTLINE_RIGHT_X__",
    &(navigation_x + navigation_outline).to_string(),
  );
  let theme = theme.replace("__MAIN_FONT_SIZE__", &main_font_size.to_string());
  let theme = theme.replace("__TERMINAL_FONT_SIZE__", &TERMINAL_FONT_SIZE.to_string());

  fs::write("./ddgc/theme.txt", theme)?;

  Ok(())
}

// https://stackoverflow.com/a/29557629
fn round_to_nearest_multiple(value: u32, multiple: u32) -> u32 {
  ((value + multiple / 2) / multiple) * multiple
}
