[![Rust](https://github.com/floooh/sokol-rust/actions/workflows/main.yml/badge.svg)](https://github.com/floooh/sokol-rust/actions/workflows/main.yml)

## sokol-rust
Auto-generated Rust bindings for the [sokol headers](https://github.com/floooh/sokol).

Add `sokol-rust` as a dependency to your `Cargo.toml` as such:
```toml
sokol = { version="*", git="https://github.com/floooh/sokol-rust.git" }
```

Check out the `examples/` folder for more examples. Here is `examples/clear/clear.rs`:
```rust
use sokol::app as sapp;
use sokol::gfx as sg;

struct State {
    pass_action: sg::PassAction,
}

static mut STATE: State = State {
    pass_action: sg::PassAction::new(),
};

extern "C" fn init() {
    let state = unsafe { &mut STATE };

    sg::setup(&sg::Desc {
        context: sokol::glue::context(),
        ..Default::default()
    });

    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

    let g = state.pass_action.colors[0].value.g + 0.01;
    state.pass_action.colors[0].value.g = if g > 1.0 { 0.0 } else { g };

    let (width, height) = (sapp::width(), sapp::height());

    sg::begin_default_pass(&state.pass_action, width, height);
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    sg::shutdown()
}

fn main() {
    let window_title = b"clear\0".as_ptr() as _;

    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        frame_cb: Some(frame),
        window_title,
        width: 800,
        height: 600,
        sample_count: 4,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
```

## Dependencies
The rust compiler and cargo can be installed using [rustup](https://rustup.rs/)

The same dependencies apply as with sokol normally for each platform

## Building with cargo
Cargo will compile and link the sokol headers automatically durnig compilation thanks to the buildscript `build.rs`

## Examples
Not all examples have been translated to rust yet, but you can check the onces that have been in the `examples` directory.

You can compile all examples using the following command:
```console
cargo build --all-targets
```

Build and run individual examples as such:
```console
cargo run --example clear
cargo run --example cube
cargo run --example mrt
cargo run --example debugtext
cargo run --example sgl-context
cargo run --example sgl-points
cargo run --example blend
cargo run --example audio
cargo run --example instancing
cargo run --example userdata
```

>NOTE: The imgui sample currently doesn't work, see the note about lib.rs here [here](https://github.com/floooh/sokol-rust/issues/4#issuecomment-1489105274), and [this issue](https://github.com/floooh/sokol-rust/issues/3) for a clean solution.

To run the imgui example, you need to go into it's directory:
```console
cd examples/imgui
cargo run
```

## Wasm/Emscripten
To compile for wasm, you will need the emcc compiler which you can get at https://github.com/emscripten-core/emsdk

You can then compile the examples like such:

```console
cargo build --target wasm32-unknown-emscripten --example texcube
```

You will then need to create an html page which imports the game. Checkout `test.html` for how this can be done. It is specifically setup to
run the texcube example in debug mode.

It can be served with `basic-http-server`:
```console
cargo install basic-http-server
basic-http-server .
# .. now go to localhost:4000/test.html
```

## Shaders
Checkout [sokol-tools](https://github.com/floooh/sokol-tools) for a sokol shader pipeline! It supports these rust bindings and all shaders in the examples folder
here have been compiled using it with `-f sokol_rust`!

## License and attributinos
This code is released under the zlib license (see `LICENSE` for info). Parts of `gen_rust.py` and `build.rs` have been copied and modified from
the zig-bindings (https://github.com/floooh/sokol-odin/) and odin-bindings (https://github.com/floooh/sokol-odin/) for sokol.

The sokol headers are created by Andre Weissflog (floooh) and sokol is released under its own license here: https://github.com/floooh/sokol/blob/master/LICENSE

cimgui https://github.com/cimgui/cimgui is released under the MIT license

The old rust bindings by Daniel Ludwig (code-disaster) https://github.com/code-disaster/sokol-rs were used to figure out the `build.rs` script and it was released under the MIT license.
