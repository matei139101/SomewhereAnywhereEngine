#!/bin/bash

set -e

SRC_DIR="src/shaders"
OUT_DIR="src/shaders/compiled"

mkdir -p "$OUT_DIR"

for shader in "$SRC_DIR"/*.{vert,frag}; do
    if [[ -f "$shader" ]]; then
        filename=$(basename "$shader")
        out_file="$OUT_DIR/${filename}.spv"
        echo "Compiling $shader -> $out_file"
        glslangValidator -V "$shader" -o "$out_file"
    fi
done

echo "All shaders compiled successfully."
