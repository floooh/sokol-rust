//------------------------------------------------------------------------------
//  saudio.rs
//  Test sokol-audio Rust bindings
//------------------------------------------------------------------------------

use sokol::{app as sapp, audio as saudio, gfx as sg, glue as sglue, log as slog};

const NUM_SAMPLES: usize = 32;

struct State {
    pub pass_action: sg::PassAction,
    pub even_odd: u32,
    pub sample_pos: usize,
    pub samples: [f32; NUM_SAMPLES],
}

static mut STATE: State = State {
    pass_action: sg::PassAction::new(),
    even_odd: 0,
    sample_pos: 0,
    samples: [0.0; NUM_SAMPLES],
};

extern "C" fn init() {
    let state = unsafe { &mut STATE };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
    saudio::setup(&saudio::Desc {
        logger: saudio::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 },
        ..Default::default()
    };
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

    let num_frames = saudio::expect();

    for _ in 0..num_frames {
        state.even_odd += 1;
        state.sample_pos += 1;

        if state.sample_pos == NUM_SAMPLES {
            state.sample_pos = 0;
            let _ = saudio::push(&(state.samples[0]), NUM_SAMPLES as _);
        }

        state.samples[state.sample_pos] = if 0 != (state.even_odd & 0x20) { 0.1 } else { -0.1 };
    }

    sg::begin_pass(&sg::Pass {
        action: state.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    saudio::shutdown();
    sg::shutdown();
}

pub fn main() {
    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        frame_cb: Some(frame),
        cleanup_cb: Some(cleanup),
        width: 640,
        height: 480,
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        window_title: b"saudio\0".as_ptr() as _,
        logger: sapp::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
}
