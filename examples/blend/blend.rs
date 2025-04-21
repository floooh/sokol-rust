//------------------------------------------------------------------------------
//  cube/cube.rs
//
//  Test/demonstrate blend modes.
//------------------------------------------------------------------------------

mod math;
mod shader;

use std::ffi;

use math as m;
use sokol::{app as sapp, gfx as sg, glue as sglue};

const NUM_BLEND_FACTORS: usize = 15;

struct State {
    pass_action: sg::PassAction,
    bind: sg::Bindings,
    pips: [[sg::Pipeline; NUM_BLEND_FACTORS]; NUM_BLEND_FACTORS],
    bg_pip: sg::Pipeline,
    r: f32,
    quad_vs_params: shader::QuadVsParams,
    bg_fs_params: shader::BgFsParams,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        pipeline_pool_size: (NUM_BLEND_FACTORS * NUM_BLEND_FACTORS + 1) as _,
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // a default pass action which does not clear, since the entire screen is overwritten anyway
    state.pass_action.colors[0].load_action = sg::LoadAction::Dontcare;
    state.pass_action.depth.load_action = sg::LoadAction::Dontcare;
    state.pass_action.stencil.load_action = sg::LoadAction::Dontcare;

    // a quad vertex buffer
    #[rustfmt::skip]
    const VERTICES: &[f32] = &[
        // pos            color
        -1.0, -1.0, 0.0,  1.0, 0.0, 0.0, 0.5,
         1.0, -1.0, 0.0,  0.0, 1.0, 0.0, 0.5,
        -1.0,  1.0, 0.0,  0.0, 0.0, 1.0, 0.5,
         1.0,  1.0, 0.0,  1.0, 1.0, 0.0, 0.5,
    ];
    state.bind.vertex_buffers[0] =
        sg::make_buffer(&sg::BufferDesc { data: sg::slice_as_range(VERTICES), ..Default::default() });

    // shader and pipeline object for rendering the background quad
    state.bg_pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::bg_shader_desc(sg::query_backend())),
        // we use the same vertex buffer as for the colored 3D quads,
        // but only the first two floats from the position, need to
        // provide a stride to skip the gap to the next vertex
        layout: {
            let mut layout = sg::VertexLayoutState::new();
            layout.buffers[0].stride = 28;
            layout.attrs[shader::ATTR_BG_POSITION].format = sg::VertexFormat::Float2;
            layout
        },
        primitive_type: sg::PrimitiveType::TriangleStrip,
        ..Default::default()
    });

    // a shader for the blended quads
    let quad_shd = sg::make_shader(&shader::quad_shader_desc(sg::query_backend()));

    // one pipeline object per blend-factor combination
    let mut pip_desc = sg::PipelineDesc {
        layout: {
            let mut layout = sg::VertexLayoutState::new();
            layout.attrs[shader::ATTR_QUAD_POSITION].format = sg::VertexFormat::Float3;
            layout.attrs[shader::ATTR_QUAD_COLOR0].format = sg::VertexFormat::Float4;
            layout
        },
        shader: quad_shd,
        primitive_type: sg::PrimitiveType::TriangleStrip,
        blend_color: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };
    for src in 0..NUM_BLEND_FACTORS {
        for dst in 0..NUM_BLEND_FACTORS {
            pip_desc.colors[0].blend = sg::BlendState {
                enabled: true,
                src_factor_rgb: unsafe { std::mem::transmute((src + 1) as u32) },
                dst_factor_rgb: unsafe { std::mem::transmute((dst + 1) as u32) },
                src_factor_alpha: sg::BlendFactor::One,
                dst_factor_alpha: sg::BlendFactor::Zero,
                ..Default::default()
            };
            state.pips[src][dst] = sg::make_pipeline(&pip_desc);
        }
    }
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    let t = (sapp::frame_duration() * 60.0) as f32;
    state.r += 0.6 * t;
    state.bg_fs_params.tick += 1.0 * t;

    // view-projection matrix
    let proj = m::persp_mat4(90.0, sapp::widthf() / sapp::heightf(), 0.01, 100.0);
    let view = m::lookat_mat4(m::vec3(0.0, 0.0, 25.0), m::Vec3::ZERO, m::Vec3::UP);
    let view_proj = m::mul_mat4(proj, view);

    // start rendering
    sg::begin_pass(&sg::Pass {
        action: state.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });

    // draw a background quad
    sg::apply_pipeline(state.bg_pip);
    sg::apply_bindings(&state.bind);
    sg::apply_uniforms(shader::UB_BG_FS_PARAMS, &sg::value_as_range(&state.bg_fs_params));
    sg::draw(0, 4, 1);

    // draw the blended quads
    let r0 = state.r;
    for src in 0..NUM_BLEND_FACTORS {
        for dst in 0..NUM_BLEND_FACTORS {
            // compute model-view-proj matrix
            let rm = m::rotate_mat4(r0, m::vec3(0.0, 1.0, 0.0));
            let x = (dst as i32 - (NUM_BLEND_FACTORS / 2) as i32) as f32 * 3.0;
            let y = (src as i32 - (NUM_BLEND_FACTORS / 2) as i32) as f32 * 2.2;
            let model = m::mul_mat4(m::translate_mat4(m::vec3(x, y, 0.0)), rm);
            state.quad_vs_params.mvp = m::mul_mat4(view_proj, model);

            sg::apply_pipeline(state.pips[src][dst]);
            sg::apply_bindings(&state.bind);
            sg::apply_uniforms(shader::UB_QUAD_VS_PARAMS, &sg::value_as_range(&state.quad_vs_params));
            sg::draw(0, 4, 1);
        }
    }
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sg::shutdown();

    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    let state = Box::new(State {
        pass_action: sg::PassAction::new(),
        bind: sg::Bindings::new(),
        pips: [[sg::Pipeline::new(); NUM_BLEND_FACTORS]; NUM_BLEND_FACTORS],
        bg_pip: sg::Pipeline::new(),
        r: 0.0,
        quad_vs_params: shader::QuadVsParams { mvp: [[0.0; 4]; 4] },
        bg_fs_params: shader::BgFsParams { tick: 0.0, _pad_4: [0; 12] },
    });

    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        user_data,
        width: 800,
        height: 600,
        sample_count: 4,
        window_title: c"blend.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        ..Default::default()
    });
}
