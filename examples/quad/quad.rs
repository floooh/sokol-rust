//------------------------------------------------------------------------------
//  quad.rs
//
//  Simple 2D rendering with vertex- and index-buffer.
//------------------------------------------------------------------------------

mod shader;

use std::ffi;

use sokol::{
    app as sapp,
    gfx::{self as sg, ColorAttachmentAction, VertexFormat},
    glue as sglue,
};

struct State {
    bind: sg::Bindings,
    pip: sg::Pipeline,
    pass_action: sg::PassAction,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // a vertex buffer
    state.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        #[rustfmt::skip]
        data: sg::value_as_range::<[f32; _]>(&[
            // positions     colors
            -0.5,  0.5, 0.5, 1.0, 0.0, 0.0, 1.0,
             0.5,  0.5, 0.5, 0.0, 1.0, 0.0, 1.0,
             0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0,
            -0.5, -0.5, 0.5, 1.0, 1.0, 0.0, 1.0,
        ]),
        ..Default::default()
    });

    // an index buffer
    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        usage: sg::BufferUsage { index_buffer: true, ..Default::default() },
        data: sg::value_as_range::<[u16; _]>(&[0, 1, 2, 0, 2, 3]),
        ..Default::default()
    });

    // a shader and pipeline state object
    state.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::quad_shader_desc(sg::query_backend())),
        layout: {
            let mut l = sg::VertexLayoutState::new();
            l.attrs[shader::ATTR_QUAD_POSITION].format = VertexFormat::Float3;
            l.attrs[shader::ATTR_QUAD_COLOR0].format = VertexFormat::Float4;
            l
        },
        index_type: sg::IndexType::Uint16,
        ..Default::default()
    });

    // clear to black
    state.pass_action.colors[0] = ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    // default pass-action clears to grey
    sg::begin_pass(&sg::Pass {
        action: state.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sg::apply_pipeline(state.pip);
    sg::apply_bindings(&state.bind);
    sg::draw(0, 6, 1);
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sg::shutdown();

    // Convert the user_data back to an owned ptr so that it gets dropped correctly
    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    // Heap allocated state struct, passed to app callbacks via user_data
    let state = Box::new(State {
        pip: sg::Pipeline::new(),
        bind: sg::Bindings::new(),
        pass_action: sg::PassAction::new(),
    });

    // Forget the ownership so we can pass as a user_data pointer
    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        user_data,
        width: 640,
        height: 480,
        window_title: c"quad.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },

        ..Default::default()
    });
}
