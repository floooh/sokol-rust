# HACK HACK HACK, assumes Mac
../sokol-tools-bin/bin/osx_arm64/sokol-shdc -i examples/blend/shader.glsl -o examples/blend/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
../sokol-tools-bin/bin/osx_arm64/sokol-shdc -i examples/cube/shader.glsl -o examples/cube/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
../sokol-tools-bin/bin/osx_arm64/sokol-shdc -i examples/instancing/shader.glsl -o examples/instancing/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
../sokol-tools-bin/bin/osx_arm64/sokol-shdc -i examples/mrt/shader.glsl -o examples/mrt/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
../sokol-tools-bin/bin/osx_arm64/sokol-shdc -i examples/texcube/shader.glsl -o examples/texcube/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
../sokol-tools-bin/bin/osx_arm64/sokol-shdc -i examples/vertexpull/shader.glsl -o examples/vertexpull/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust
