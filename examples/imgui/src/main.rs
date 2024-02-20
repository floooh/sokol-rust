//------------------------------------------------------------------------------
//  sgl-points/sgl-points.rs
//
//  Test point rendering with sokol/gl
//------------------------------------------------------------------------------

use sokol::app as sapp;
use sokol::gfx as sg;
use sokol::glue as sglue;
use sokol::imgui as simgui;

extern "C" fn init() {
    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger {
            func: Some(sokol::log::slog_func),
            ..Default::default()
        },
        ..Default::default()
    });

    simgui::setup(&simgui::Desc {
        ..Default::default()
    });
}

fn imgui_frame() {
    unsafe {
        let mut test = false;
        imgui_sys::igBegin(b"Demo window\0".as_ptr() as _, &mut test as _, 0);
        imgui_sys::igButton(
            b"Hello!\0".as_ptr() as _,
            imgui_sys::ImVec2 { x: 0.0, y: 0.0 },
        );
        imgui_sys::igEnd();
    }
}

extern "C" fn frame() {
    simgui::new_frame(&simgui::FrameDesc {
        width: sapp::width(),
        height: sapp::height(),
        delta_time: sapp::frame_duration(),
        dpi_scale: 1.0,

        ..Default::default()
    });

    imgui_frame();

    let mut pass_action = sg::PassAction::new();
    pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color {
            r: 0.3,
            g: 0.3,
            b: 0.3,
            a: 1.0,
        },
        ..Default::default()
    };
    sg::begin_pass(&sg::Pass {
        action: pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    simgui::render();
    sg::end_pass();
    sg::commit();
}

extern "C" fn event(event: *const sapp::Event) {
    let event: &sapp::Event = unsafe { &*event };
    let _handled = simgui::handle_event(event);
}

extern "C" fn cleanup() {
    simgui::shutdown();
    sg::shutdown();
}

fn main() {
    let window_title = b"imgui\0".as_ptr() as *const _;

    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        frame_cb: Some(frame),
        cleanup_cb: Some(cleanup),
        event_cb: Some(event),

        width: 512,
        height: 512,
        sample_count: 4,
        window_title,
        logger: sapp::Logger {
            func: Some(sokol::log::slog_func),
            ..Default::default()
        },
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
