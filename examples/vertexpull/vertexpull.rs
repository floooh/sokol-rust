//------------------------------------------------------------------------------
//  vertexpull.rs
//
//  Vertex-pulling from a storage buffer.
//------------------------------------------------------------------------------

mod math;
mod shader;

use std::ffi;

use math as m;
use shader as shd;
use sokol::{app as sapp, gfx as sg, glue as sglue, log as slog};

struct State {
    rx: f32,
    ry: f32,
    pip: sg::Pipeline,
    bind: sg::Bindings,
    pass_action: sg::PassAction,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });

    // if no storage buffers supported by 3D backend, set clear color to red and return now
    if !sg::query_features().compute {
        state.pass_action.colors[0] = sg::ColorAttachmentAction {
            load_action: sg::LoadAction::Clear,
            clear_value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
            ..Default::default()
        };
        return;
    }
    // otherwise set to regular clear color
    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.75, g: 0.5, b: 0.25, a: 1.0 },
        ..Default::default()
    };

    // Vertices in a storage buffer, we *could* use the code-generated
    // SbVertex struct here, but the initialization code would look so gnarly
    // that I'm skipping that (seriously, wtf Rust?). Be aware that there's
    // 4 bytes of padding between the pos and color!
    #[rustfmt::skip]
    const VERTICES: &[f32] = &[
        // pos              padding  color
        -1.0, -1.0, -1.0,   0.0,     1.0, 0.0, 0.0, 1.0,
         1.0, -1.0, -1.0,   0.0,     1.0, 0.0, 0.0, 1.0,
         1.0,  1.0, -1.0,   0.0,     1.0, 0.0, 0.0, 1.0,
        -1.0,  1.0, -1.0,   0.0,     1.0, 0.0, 0.0, 1.0,
        -1.0, -1.0,  1.0,   0.0,     0.0, 1.0, 0.0, 1.0,
         1.0, -1.0,  1.0,   0.0,     0.0, 1.0, 0.0, 1.0,
         1.0,  1.0,  1.0,   0.0,     0.0, 1.0, 0.0, 1.0,
        -1.0,  1.0,  1.0,   0.0,     0.0, 1.0, 0.0, 1.0,
        -1.0, -1.0, -1.0,   0.0,     0.0, 0.0, 1.0, 1.0,
        -1.0,  1.0, -1.0,   0.0,     0.0, 0.0, 1.0, 1.0,
        -1.0,  1.0,  1.0,   0.0,     0.0, 0.0, 1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0,     0.0, 0.0, 1.0, 1.0,
         1.0, -1.0, -1.0,   0.0,     1.0, 0.5, 0.0, 1.0,
         1.0,  1.0, -1.0,   0.0,     1.0, 0.5, 0.0, 1.0,
         1.0,  1.0,  1.0,   0.0,     1.0, 0.5, 0.0, 1.0,
         1.0, -1.0,  1.0,   0.0,     1.0, 0.5, 0.0, 1.0,
        -1.0, -1.0, -1.0,   0.0,     0.0, 0.5, 1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0,     0.0, 0.5, 1.0, 1.0,
         1.0, -1.0,  1.0,   0.0,     0.0, 0.5, 1.0, 1.0,
         1.0, -1.0, -1.0,   0.0,     0.0, 0.5, 1.0, 1.0,
        -1.0,  1.0, -1.0,   0.0,     1.0, 0.0, 0.5, 1.0,
        -1.0,  1.0,  1.0,   0.0,     1.0, 0.0, 0.5, 1.0,
         1.0,  1.0,  1.0,   0.0,     1.0, 0.0, 0.5, 1.0,
         1.0,  1.0, -1.0,   0.0,     1.0, 0.0, 0.5, 1.0,
    ];
    state.bind.views[shd::VIEW_SSBO] = sg::make_view(&sg::ViewDesc {
        storage_buffer: sg::BufferViewDesc {
            buffer: sg::make_buffer(&sg::BufferDesc {
                usage: sg::BufferUsage { storage_buffer: true, ..Default::default() },
                data: sg::slice_as_range(VERTICES),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    });

    // a regular index buffer
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
        usage: sg::BufferUsage { index_buffer: true, ..Default::default() },
        data: sg::slice_as_range(INDICES),
        ..Default::default()
    });

    // shader and pipeline object, note that the pipeline object has not vertex layout
    // (not needed when pulling vertices from a storage buffer)
    state.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::vertexpull_shader_desc(sg::query_backend())),
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
    let state = unsafe { &mut *(user_data as *mut State) };

    let t = (sapp::frame_duration() * 60.0) as f32;
    state.rx += 1.0 * t;
    state.ry += 2.0 * t;

    // vertex shader uniform with model-view-projection matrix
    let vs_params = shader::VsParams { mvp: compute_mvp(state.rx, state.ry) };

    sg::begin_pass(&sg::Pass {
        action: state.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    if sg::query_features().compute {
        sg::apply_pipeline(state.pip);
        sg::apply_bindings(&state.bind);
        sg::apply_uniforms(shd::UB_VS_PARAMS, &sg::value_as_range(&vs_params));
        sg::draw(0, 36, 1);
    }
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

    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    let state = Box::new(State {
        rx: 0.0,
        ry: 0.0,
        pip: sg::Pipeline::new(),
        bind: sg::Bindings::new(),
        pass_action: sg::PassAction::new(),
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
        window_title: c"vertexpull.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },

        ..Default::default()
    });
}
