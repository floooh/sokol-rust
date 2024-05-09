#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SokolBackend {
    D3d11,
    Metal,
    Gl,
    Gles2,
    Gles3,
    Wgpu,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BuildTarget {
    Linux,
    Windows,
    Macos,
    WasmEmscripten,
}

#[derive(Clone)]
struct CompilationConfig {
    is_msvc: bool,
    desired_backend: String,
    debug_info_requested: bool,
    force_egl: bool,
    is_debug_build: bool,
    wayland_desired: bool,
    build_target: BuildTarget,
}

fn get_compilation_config(tool: &cc::Tool) -> CompilationConfig {
    let desired_backend = std::env::var("SOKOL_BACKEND").ok().unwrap_or("AUTO".to_owned());
    let debug_info_requested = std::env::var("SOKOL_DEBUG").ok().is_some();
    let is_msvc = tool.is_like_msvc();
    let wayland_desired = std::env::var("SOKOL_WAYLAND").is_ok();
    let force_egl = std::env::var("SOKOL_FORCE_EGL").is_ok();
    let is_debug_build = cfg!(debug_assertions);

    /*
        HACK: #[cfg(target_arch)] doesnt give the build target in build.rs,
              only the host target. So we need to read the TARGET env variable to
              be able to cross compile. This is unacceptable.
    */
    let target_var = std::env::var("TARGET").unwrap();
    let split = target_var.split('-').collect::<Vec<_>>();
    let is_wasm_hack = split.contains(&"wasm32") || split.contains(&"emscripten");

    let build_target = if is_wasm_hack {
        BuildTarget::WasmEmscripten
    } else if cfg!(target_os = "windows") && is_msvc {
        BuildTarget::Windows
    } else if cfg!(target_os = "macos") {
        BuildTarget::Macos
    } else if cfg!(target_os = "linux") {
        BuildTarget::Linux
    } else {
        todo!("Unsupported build target");
    };

    CompilationConfig {
        is_msvc,
        desired_backend,
        wayland_desired,
        force_egl,
        is_debug_build,
        debug_info_requested,
        build_target,
    }
}

fn select_sokol_backend(build: &mut cc::Build, config: &CompilationConfig) -> SokolBackend {
    println!("cargo:rerun-if-env-changed=SOKOL_BACKEND");
    println!("cargo:rerun-if-env-changed=SOKOL_WAYLAND");
    println!("cargo:rerun-if-env-changed=SOKOL_FORCE_EGL");
    println!("cargo:rerun-if-env-changed=SOKOL_DEBUG");

    let backend = match &config.desired_backend[..] {
        "AUTO" => match config.build_target {
            BuildTarget::Linux => SokolBackend::Gl,
            BuildTarget::Windows => SokolBackend::D3d11,
            BuildTarget::Macos => SokolBackend::Metal,
            BuildTarget::WasmEmscripten => SokolBackend::Gles3,
        },

        "D3D11" => SokolBackend::D3d11,
        "METAL" => SokolBackend::Metal,
        "GL" => SokolBackend::Gl,
        "GLES2" => SokolBackend::Gles2,
        "GLES3" => SokolBackend::Gles3,
        "WGPU" => SokolBackend::Wgpu,

        _ => panic!("Unknown SOKOL_BACKEND: {}", &config.desired_backend),
    };

    match backend {
        SokolBackend::D3d11 => {
            build.define("SOKOL_D3D11", None);
        },
        SokolBackend::Metal => {
            build.define("SOKOL_METAL", None);
        },
        SokolBackend::Gles3 => {
            build.define("SOKOL_GLES3", None);
        },
        SokolBackend::Gles2 => {
            build.define("SOKOL_GLES2", None);
        },
        SokolBackend::Gl => {
            build.define("SOKOL_GLCORE", None);
        },
        SokolBackend::Wgpu => {
            build.define("SOKOL_WGPU", None);
        },
    }

    backend
}

fn make_sokol() {
    let mut build = cc::Build::new();
    let tool = build.try_get_compiler().unwrap();
    let config = get_compilation_config(&tool);

    const BASE_C_DIR: &str = "src/sokol/c/";

    if !config.is_debug_build {
        build.define("NDEBUG", None);
        build.opt_level(2);
    }

    let backend = select_sokol_backend(&mut build, &config);

    let files = [
        "sokol_log.c", "sokol_app.c", "sokol_gfx.c", "sokol_glue.c", "sokol_time.c", "sokol_audio.c",
        "sokol_gl.c", "sokol_debugtext.c", "sokol_shape.c",
    ];

    //
    // include paths
    //
    build.include(BASE_C_DIR);

    build.define("IMPL", None);

    //
    // silence some warnings
    //
    build.flag_if_supported("-Wno-unused-parameter");
    build.flag_if_supported("-Wno-missing-field-initializers");

    if config.debug_info_requested {
        build.define("_DEBUG", None).define("SOKOL_DEBUG", None);
    }

    for file in &files {
        let file = format!("{BASE_C_DIR}{file}");

        println!("cargo:rerun-if-changed={file}");

        if config.build_target == BuildTarget::Macos {
            build.flag("-ObjC");
        }

        build.file(file);
    }

    println!("cargo:rustc-link-lib=static=sokol-rust");
    println!("cargo:rustc-link-search=src/sokol/c/");
    println!("cargo:rustc-link-search=target/debug/");

    match config.build_target {
        BuildTarget::Windows => {
            if !config.is_msvc {
                build.define("_WIN32_WINNIT", Some("0x0601"));

                build
                    .flag("-D_WIN32_WINNT=0x0601")
                    .flag_if_supported("-Wno-cast-function-type")
                    .flag_if_supported("-Wno-sign-compare")
                    .flag_if_supported("-Wno-unknown-pragmas");

                println!("cargo:rustc-link-lib=static=gdi32");
                println!("cargo:rustc-link-lib=static=ole32");
                println!("cargo:rustc-link-lib=static=kernel32");
                println!("cargo:rustc-link-lib=static=user32");

                if backend == SokolBackend::D3d11 {
                    println!("cargo:rustc-link-lib=static=d3d11");
                    println!("cargo:rustc-link-lib=static=dxgi");
                }
            }
        },

        BuildTarget::Macos => {
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");

            match backend {
                SokolBackend::Metal => {
                    println!("cargo:rustc-link-lib=framework=Metal");
                    println!("cargo:rustc-link-lib=framework=MetalKit");
                },

                SokolBackend::Gl | SokolBackend::Gles2 | SokolBackend::Gles3 => {
                    println!("cargo:rustc-link-lib=framework=OpenGL");
                },

                _ => {
                    todo!("unsupported target-backend combo")
                },
            }
        },

        BuildTarget::Linux => {
            if config.wayland_desired {
                build.define("SOKOL_DISABLE_X11 ", None);
            } else {
                build.define("SOKOL_DISABLE_WAYLAND", None);
            }

            if config.force_egl {
                build.define("SOKOL_FORCE_EGL", None);
            }

            match backend {
                SokolBackend::Gles2 => {
                    println!("cargo:rustc-link-lib=dylib=glesv2");
                    assert!(config.force_egl || config.wayland_desired);
                },
                SokolBackend::Gles3 | SokolBackend::Gl => {
                    println!("cargo:rustc-link-lib=dylib=GL");
                },
                _ => {},
            }

            if config.force_egl || config.wayland_desired {
                println!("cargo:rustc-link-lib=dylib=egl");
            }

            println!("cargo:rustc-link-lib=dylib=asound");

            if config.wayland_desired {
                println!("cargo:rustc-link-lib=dylib=wayland-client");
                println!("cargo:rustc-link-lib=dylib=wayland-cursor");
                println!("cargo:rustc-link-lib=dylib=wayland-egl");
                println!("cargo:rustc-link-lib=dylib=xkbcommon");
            } else {
                println!("cargo:rustc-link-lib=dylib=X11");
                println!("cargo:rustc-link-lib=dylib=Xi");
                println!("cargo:rustc-link-lib=dylib=Xcursor");
            }
        },

        BuildTarget::WasmEmscripten => match backend {
            SokolBackend::Gles3 => {
                println!("cargo:rustc-link-arg=-sUSE_WEBGL2");
            },
            SokolBackend::Wgpu => {
                println!("cargo:rustc-link-arg=-sUSE_WEBGPU");
            },
            _ => {},
        },
    }

    build.compile("sokol-rust");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    make_sokol();
}
