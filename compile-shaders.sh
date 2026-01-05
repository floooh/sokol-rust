#!/usr/bin/env bash
set -e

if [ -z "$1" ]
then
    echo "usage: ./compile-shaders.sh [path-to-sokol-shdc]"
    exit 1
fi

shdc="$1"

$shdc -i examples/blend/shader.glsl -o examples/blend/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/cube/shader.glsl -o examples/cube/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/instancing/shader.glsl -o examples/instancing/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/mrt/shader.glsl -o examples/mrt/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/texcube/shader.glsl -o examples/texcube/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/vertexpull/shader.glsl -o examples/vertexpull/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/instancing-compute/shader.glsl -o examples/instancing-compute/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/offscreen/shader.glsl -o examples/offscreen/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/triangle/shader.glsl -o examples/triangle/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/quad/shader.glsl -o examples/quad/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
$shdc -i examples/bufferoffsets/shader.glsl -o examples/bufferoffsets/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
