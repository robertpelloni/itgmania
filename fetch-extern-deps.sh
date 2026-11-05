#!/bin/bash
# Fetch external dependencies that were removed from git submodules
# to prevent Jules recursive clone failures via stale proxy cache.
set -e
EXTERN_DIR="extern"
REPOS=(
  "IXWebSocket|https://github.com/machinezone/IXWebSocket.git"
  "ffmpeg|https://github.com/FFmpeg/FFmpeg.git"
  "hidapi|https://github.com/libusb/hidapi.git"
  "libjpeg-turbo|https://github.com/libjpeg-turbo/libjpeg-turbo.git"
  "libpng|https://github.com/pnggroup/libpng.git"
  "libtomcrypt|https://github.com/libtom/libtomcrypt.git"
  "libtommath|https://github.com/libtom/libtommath.git"
  "libusb|https://github.com/libusb/libusb-cmake.git"
  "mbedtls|https://github.com/Mbed-TLS/mbedtls.git"
  "ogg|https://github.com/xiph/ogg.git"
  "vorbis|https://github.com/xiph/vorbis.git"
  "zlib|https://github.com/madler/zlib.git"
)
mkdir -p "$EXTERN_DIR"
for entry in "${REPOS[@]}"; do
  IFS='|' read -r name url <<< "$entry"
  if [ ! -d "$EXTERN_DIR/$name" ]; then
    echo "Cloning $name..."
    git clone --depth 1 "$url" "$EXTERN_DIR/$name"
  else
    echo "Skipping $name (already exists)"
  fi
done
echo "All external dependencies fetched."
