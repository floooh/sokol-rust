[![Rust](https://github.com/floooh/sokol-rust/actions/workflows/main.yml/badge.svg)](https://github.com/floooh/sokol-rust/actions/workflows/main.yml)

## sokol-rust
Auto-generated Rust bindings for the [sokol headers](https://github.com/floooh/sokol).

Add `sokol-rust` as a dependency to your `Cargo.toml` as such:
```toml
sokol = { version="*", git="https://github.com/floooh/sokol-rust.git" }
```

Check out the `examples/` folder for more examples. Here is `examples/clear/clear.rs`:
```rust
use sokol::{app as sapp, gfx as sg, glue as sglue};
use std::ffi;

struct State {
    pass_action: sg::PassAction,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };

    let backend = sg::query_backend();
    match &backend {
        sg::Backend::Glcore | sg::Backend::Gles3 => {
            println!("Using GL Backend!");
            println!("Specifically the {:?} backend!", backend);
        },

        sg::Backend::D3d11 => {
            println!("Using D3d11 Backend!");
        },

        sg::Backend::MetalIos | sg::Backend::MetalMacos | sg::Backend::MetalSimulator => {
            println!("Using Metal Backend!");
            println!("Specifically the {:?} backend!", backend);
        },

        sg::Backend::Wgpu => {
            println!("Using Wgpu Backend!");
        },

        sg::Backend::Dummy => {
            println!("Using Dummy Backend!");
        },
    }
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    let g = state.pass_action.colors[0].clear_value.g + 0.01;
    state.pass_action.colors[0].clear_value.g = if g > 1.0 { 0.0 } else { g };

    sg::begin_pass(&sg::Pass {
        action: state.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sg::shutdown();

    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    let state = Box::new(State { pass_action: sg::PassAction::new() });

    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        user_data,
        window_title: c"clear.rs".as_ptr(),
        width: 800,
        height: 600,
        sample_count: 4,
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        ..Default::default()
    });
}
```

## Dependencies
The Rust compiler and cargo can be installed using [rustup](https://rustup.rs/)

The same dependencies apply as with sokol normally for each platform.

## Building with cargo
Cargo will compile and link the sokol headers automatically during compilation thanks to the buildscript `build.rs`

## Examples
Not all examples have been translated to Rust yet, but you can check the ones that have been in the `examples` directory.

Rust 1.77 or later is required to build the examples.

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
cargo run --example vertexpull
```

>NOTE: imgui support has been removed for now, the required cimgui submodule dependency caused trouble with
> Github Actions. We'll need to solve this some other way in the future, and in a way that works for all
> language bindings.

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
Checkout [sokol-tools](https://github.com/floooh/sokol-tools) for a sokol shader pipeline! It supports these Rust bindings and all shaders in the examples folder
here have been compiled using it with `-f sokol_rust`!

## License and attributions
This code is released under the zlib license (see `LICENSE` for info). Parts of `gen_rust.py` and `build.rs` have been copied and modified from
the zig-bindings (https://github.com/floooh/sokol-odin/) and odin-bindings (https://github.com/floooh/sokol-odin/) for sokol.

The sokol headers are created by Andre Weissflog (floooh) and sokol is released under its own license here: https://github.com/floooh/sokol/blob/master/LICENSE

cimgui https://github.com/cimgui/cimgui is released under the MIT license

The old Rust bindings by Daniel Ludwig (code-disaster) https://github.com/code-disaster/sokol-rs were used to figure out the `build.rs` script and it was released under the MIT license.
