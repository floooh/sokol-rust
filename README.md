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
        action: sg::Action::Clear,
        value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
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

## Updating the bindings
To update, place the `gen_rust.py` script inside `sokol/bindgen` and clone this repository inside there. 
Inside `gen_all.py`, import `gen_rust` like the other bindgens and call it's `gen` function the exact and
same way as the others.

```python
import gen_rust

# Rust
gen_rust.prepare()
for task in tasks:
    [c_header_path, main_prefix, dep_prefixes] = task
    gen_rust.gen(c_header_path, main_prefix, dep_prefixes)
```

I also recommend to run `cargo fmt` inside `sokol-rust` after the python script to clean up the output formatting.

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
Sokol's own shader compiler does not yet have support for generating helper files in rust, though they can quite easily
be translated into rust manually. See `examples/mrt/shader.rs` or `examples/cube/shader.rs` for examples on how this can been done.

I have started a work-in-progress version of the shader compiler can be found in `sokolrust.cc`. I have successfully used it to translate all the shaders that
are required to run the examples here. If you want to try it out yourself, here is some rough guidelines:

1. Place `sokolrust.cc` inside the `src/shdc/` folder of the [sokol-tools](https://github.com/floooh/sokol-tools)
2. Make some slight modifications to the code-base:
```cpp
// .. in main.cc switch case, add a case for rust
case format_t::SOKOL_RUST:
    output_err = sokolrust_t::gen(args, inp, spirvcross, bytecode);
    break;

// .. in shdc.h, add SOKOL_RUST as a format and implement the string conversions
case SOKOL_RUST:    return "sokol_rust";
// ..
else if (str == "sokol_rust") {
    return SOKOL_RUST;
}
// ..
struct sokolrust_t {
    static errmsg_t gen(const args_t& args, const input_t& inp, const std::array<spirvcross_t,slang_t::NUM>& spirvcross, const std::array<bytecode_t,slang_t::NUM>& bytecode);
};
// ..
namespace util {
    // ..
    std::string to_upper_case(const std::string& str);
    // ..
};


// .. in util.cc, add a simple helper function
std::string to_upper_case(const std::string& str) {
    return pystring::upper(str);
}
```
3. Re-compile the shader compiler and use it like normal but now with `-f sokol_rust`

I hope to add this frontend to the official compiler soon so that you don't have to apply this patch manually 

