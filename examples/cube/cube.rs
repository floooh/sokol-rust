//------------------------------------------------------------------------------
//  cube/cube.rs
//
//  Renders a rotating cube.
//------------------------------------------------------------------------------

mod math;
mod shader;

use math as m;
use sokol::{app as sapp, gfx as sg, glue as sglue};
use std::ffi;

struct State {
    rx: f32,
    ry: f32,

    pip: sg::Pipeline,
    bind: sg::Bindings,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // cube vertex buffer
    #[rustfmt::skip]
    const VERTICES: &[f32] = &[
        -1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
         1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
         1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
        -1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,

        -1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
         1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
         1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
        -1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,

        -1.0, -1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0,  1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0,  1.0,  1.0,   0.0, 0.0, 1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0, 0.0, 1.0, 1.0,

        1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
        1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
        1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,
        1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,

        -1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
         1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
         1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,

        -1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
        -1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
         1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
         1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
    ];
    state.bind.vertex_buffers[0] =
        sg::make_buffer(&sg::BufferDesc { data: sg::slice_as_range(VERTICES), ..Default::default() });

    // create an index buffer for the cube
    #[rustfmt::skip]
    const INDICES: &[u16] = &[
        0,  1,  2,   0,  2,  3,
        6,  5,  4,   7,  6,  4,
        8,  9,  10,  8,  10, 11,
        14, 13, 12,  15, 14, 12,
        16, 17, 18,  16, 18, 19,
        22, 21, 20,  23, 22, 20,
    ];

    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(INDICES),
        _type: sg::BufferType::Indexbuffer,
        ..Default::default()
    });

    // shader and pipeline object

    state.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::cube_shader_desc(sg::query_backend())),
        layout: {
            let mut layout = sg::VertexLayoutState::new();
            layout.buffers[0].stride = 28;

            layout.attrs[shader::ATTR_VS_POSITION].format = sg::VertexFormat::Float3;
            layout.attrs[shader::ATTR_VS_COLOR0].format = sg::VertexFormat::Float4;

            layout
        },

        index_type: sg::IndexType::Uint16,
        cull_mode: sg::CullMode::Back,

        depth: sg::DepthState {
            write_enabled: true,
            compare: sg::CompareFunc::LessEqual,

            ..Default::default()
        },

        ..Default::default()
    });
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    let t = (sapp::frame_duration() * 60.0) as f32;
    state.rx += 1.0 * t;
    state.ry += 2.0 * t;

    // vertex shader uniform with model-view-projection matrix
    let vs_params = shader::VsParams { mvp: compute_mvp(state.rx, state.ry) };

    let mut pass_action = sg::PassAction::new();
    pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.25, g: 0.5, b: 0.75, a: 1.0 },
        ..Default::default()
    };

    sg::begin_pass(&sg::Pass { action: pass_action, swapchain: sglue::swapchain(), ..Default::default() });
    sg::apply_pipeline(state.pip);
    sg::apply_bindings(&state.bind);
    sg::apply_uniforms(sg::ShaderStage::Vs, shader::SLOT_VS_PARAMS, &sg::value_as_range(&vs_params));
    sg::draw(0, 36, 1);
    sg::end_pass();
    sg::commit();
}

pub fn compute_mvp(rx: f32, ry: f32) -> [[f32; 4]; 4] {
    let proj = m::persp_mat4(60.0, sapp::widthf() / sapp::heightf(), 0.01, 10.0);
    let view = m::lookat_mat4(m::vec3(0.0, 1.5, 6.0), m::Vec3::ZERO, m::Vec3::UP);
    let view_proj = m::mul_mat4(proj, view);
    let rxm = m::rotate_mat4(rx, m::vec3(1.0, 0.0, 0.0));
    let rym = m::rotate_mat4(ry, m::vec3(0.0, 1.0, 0.0));
    let model = m::mul_mat4(rxm, rym);

    m::mul_mat4(view_proj, model)
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sg::shutdown();

    // Convert the user_data back to an owned ptr so that it gets dropped correctly
    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    // Heap allocated state struct, passed to app callbacks via user_data
    let state = Box::new(State {
        rx: 0.0,
        ry: 0.0,
        pip: sg::Pipeline::new(),
        bind: sg::Bindings::new(),
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
        sample_count: 4,
        window_title: c"cube.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },

        ..Default::default()
    });
}
