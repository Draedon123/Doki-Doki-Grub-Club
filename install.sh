#!/bin/sh

read -p "Enter your display resolution in pixels, with dimensions separated by a space (e.g., 1920 1080): " width height

if [[ ! -e gen_bg ]]; then
  echo "gen_bg executable not found " >&2
  read -r -p "Press Enter to exit..."
  exit 1
fi

./gen_bg $width $height