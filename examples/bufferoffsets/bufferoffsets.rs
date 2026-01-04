//------------------------------------------------------------------------------
//  bufferoffsets.zig
//
//  Render separate geometries in vertex- and index-buffers with
//  buffer offsets.
//------------------------------------------------------------------------------

mod shader;

use std::{ffi, mem};

use sokol::{
    app as sapp,
    gfx::{self as sg, ColorAttachmentAction, VertexFormat, apply_pipeline},
    glue as sglue,
};

struct State {
    bind: sg::Bindings,
    pip: sg::Pipeline,
    pass_action: sg::PassAction,
}

#[repr(C)]
struct Vertex {
    x: f32,
    y: f32,
    r: f32,
    g: f32,
    b: f32,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // clear to a blue-ish color
    state.pass_action.colors[0] = ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.5, g: 0.5, b: 1.0, a: 1.0 },
        ..Default::default()
    };

    // a 2D triangle and quad in 1 vertex buffer and 1 index buffer
    state.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        data: sg::value_as_range::<[Vertex; _]>(&[
            // triangle vertices
            Vertex { x: 0.0, y: 0.55, r: 1.0, g: 0.0, b: 0.0 },
            Vertex { x: 0.25, y: 0.05, r: 0.0, g: 1.0, b: 0.0 },
            Vertex { x: -0.25, y: 0.05, r: 0.0, g: 0.0, b: 1.0 },
            // quad vertices
            Vertex { x: -0.25, y: -0.05, r: 0.0, g: 0.0, b: 1.0 },
            Vertex { x: 0.25, y: -0.05, r: 0.0, g: 1.0, b: 0.0 },
            Vertex { x: 0.25, y: -0.55, r: 1.0, g: 0.0, b: 0.0 },
            Vertex { x: -0.25, y: -0.55, r: 1.0, g: 1.0, b: 0.0 },
        ]),
        ..Default::default()
    });

    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        usage: sg::BufferUsage { index_buffer: true, ..Default::default() },
        #[rustfmt::skip]
        data: sg::value_as_range::<[u16; _]>(&[
            // triangle indices
            0, 1, 2,
            // quad indices
            0, 1, 2,
            0, 2, 3,
        ]),
        ..Default::default()
    });

    // a shader and pipeline object
    state.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::bufferoffsets_shader_desc(sg::query_backend())),
        layout: {
            let mut l = sg::VertexLayoutState::new();
            l.attrs[shader::ATTR_BUFFEROFFSETS_POSITION].format = VertexFormat::Float2;
            l.attrs[shader::ATTR_BUFFEROFFSETS_COLOR0].format = VertexFormat::Float3;
            l
        },
        index_type: sg::IndexType::Uint16,
        ..Default::default()
    });
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

    // render the triangle
    state.bind.vertex_buffer_offsets[0] = 0;
    state.bind.index_buffer_offset = 0;
    sg::apply_bindings(&state.bind);
    sg::draw(0, 3, 1);

    // render the quad
    state.bind.vertex_buffer_offsets[0] = 3 * mem::size_of::<Vertex>() as i32;
    state.bind.index_buffer_offset = 3 * mem::size_of::<u16>() as i32;
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
        width: 800,
        height: 600,
        window_title: c"bufferoffsets.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },

        ..Default::default()
    });
}
