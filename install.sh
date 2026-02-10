#!/bin/sh

read -p "Enter your display resolution in pixels, with dimensions separated by a space (e.g., 1920 1080): " width height

if [[ ! -e generate ]]; then
  echo "generate executable not found " >&2
  read -r -p "Press Enter to exit..."
  exit 1
fi

./generate $width $height

mv ./background.png ./theme/background.png
