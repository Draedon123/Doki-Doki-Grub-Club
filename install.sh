#!/bin/sh

read -p "Enter your display resolution in pixels, with dimensions separated by a space (e.g., 1920 1080): " width height

if [[ ! -e generate ]]; then
  echo "generate executable not found " >&2
  read -r -p "Press Enter to exit..."
  exit 1
fi

if [[ ! -d ddgc ]]; then
  mkdir ddgc
fi

./generate $width $height

if [[ -d /boot/grub/themes/ddgc ]]; then
  rm -r /boot/grub/themes/ddgc
fi

cp -r ./ddgc /boot/grub/themes

update-grub