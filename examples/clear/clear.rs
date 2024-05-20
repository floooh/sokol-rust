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
