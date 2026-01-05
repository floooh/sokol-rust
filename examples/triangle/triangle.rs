//------------------------------------------------------------------------------
//  triangle.rs
//
//  Vertex buffer, shader, pipeline state object.
//------------------------------------------------------------------------------

mod shader;

use std::ffi;

use sokol::{
    app as sapp,
    gfx::{self as sg, VertexFormat},
    glue as sglue,
};

struct State {
    bind: sg::Bindings,
    pip: sg::Pipeline,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // create vertex buffer with triangle vertices
    state.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        #[rustfmt::skip]
        data: sg::value_as_range::<[f32; _]>(&[
             // positions    colors
             0.0,  0.5, 0.5, 1.0, 0.0, 0.0, 1.0,
             0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0,
        ]),
        ..Default::default()
    });

    // create a shader and pipeline object
    state.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::triangle_shader_desc(sg::query_backend())),
        layout: {
            let mut l = sg::VertexLayoutState::new();
            l.attrs[shader::ATTR_TRIANGLE_POSITION].format = VertexFormat::Float3;
            l.attrs[shader::ATTR_TRIANGLE_COLOR0].format = VertexFormat::Float4;
            l
        },
        ..Default::default()
    });
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    // default pass-action clears to grey
    sg::begin_pass(&sg::Pass { swapchain: sglue::swapchain(), ..Default::default() });
    sg::apply_pipeline(state.pip);
    sg::apply_bindings(&state.bind);
    sg::draw(0, 3, 1);
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
    let state = Box::new(State { pip: sg::Pipeline::new(), bind: sg::Bindings::new() });

    // Forget the ownership so we can pass as a user_data pointer
    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        user_data,
        width: 640,
        height: 480,
        window_title: c"triangle.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },

        ..Default::default()
    });
}
