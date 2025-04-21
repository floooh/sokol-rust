//------------------------------------------------------------------------------
//  instancing-compute.rs
//
//  Like instancing.rs, but update particles with compute shader.
//------------------------------------------------------------------------------

mod math;
mod shader;

use std::ffi;

use math as m;
use sokol::{app as sapp, gfx as sg, glue as sglue, log as slog};

const MAX_PARTICLES: usize = 512 * 1024;
const NUM_PARTICLES_EMITTED_PER_FRAME: usize = 10;

struct ComputeState {
    pip: sg::Pipeline,
    bind: sg::Bindings,
}

struct DisplayState {
    pip: sg::Pipeline,
    bind: sg::Bindings,
    pass_action: sg::PassAction,
}

struct State {
    num_particles: usize,
    ry: f32,
    compute: ComputeState,
    display: DisplayState,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });

    // if compute shaders not supported, clear to red and early out
    if !sg::query_features().compute {
        state.display.pass_action.colors[0] = sg::ColorAttachmentAction {
            load_action: sg::LoadAction::Clear,
            clear_value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
            ..Default::default()
        };
        return;
    }
    // regular clear color
    state.display.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.1, b: 0.2, a: 1.0 },
        ..Default::default()
    };

    // a zero-initialized storage buffer for the particle state
    let sbuf = sg::make_buffer(&sg::BufferDesc {
        _type: sg::BufferType::Storagebuffer,
        size: MAX_PARTICLES * std::mem::size_of::<shader::Particle>(),
        ..Default::default()
    });
    state.compute.bind.storage_buffers[shader::SBUF_CS_SSBO] = sbuf;
    state.display.bind.storage_buffers[shader::SBUF_VS_SSBO] = sbuf;

    // a compute shader and pipeline object for updating the particle state
    state.compute.pip = sg::make_pipeline(&sg::PipelineDesc {
        compute: true,
        shader: sg::make_shader(&shader::update_shader_desc(sg::query_backend())),
        ..Default::default()
    });

    // vertex and index buffer for particle geometry
    let r = 0.05;
    #[rustfmt::skip]
    let vertices: &[f32] = &[
        // positions        colors
        0.0,   -r, 0.0,     1.0, 0.0, 0.0, 1.0,
           r, 0.0, r,       0.0, 1.0, 0.0, 1.0,
           r, 0.0, -r,      0.0, 0.0, 1.0, 1.0,
          -r, 0.0, -r,      1.0, 1.0, 0.0, 1.0,
          -r, 0.0, r,       0.0, 1.0, 1.0, 1.0,
        0.0,    r, 0.0,     1.0, 0.0, 1.0, 1.0,
    ];
    state.display.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(vertices),
        _type: sg::BufferType::Vertexbuffer,
        ..Default::default()
    });
    #[rustfmt::skip]
    let indices: &[u16] = &[
        0, 1, 2,    0, 2, 3,    0, 3, 4,    0, 4, 1,
        5, 1, 2,    5, 2, 3,    5, 3, 4,    5, 4, 1,
    ];
    state.display.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        _type: sg::BufferType::Indexbuffer,
        data: sg::slice_as_range(indices),
        ..Default::default()
    });

    // shader and pipeline object for rendering the particles, this uses
    // the compute-updated storage buffer to provide the particle positions
    state.display.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::display_shader_desc(sg::query_backend())),
        layout: sg::VertexLayoutState {
            attrs: {
                let mut attrs = [sg::VertexAttrState::new(); sg::MAX_VERTEX_ATTRIBUTES];
                attrs[shader::ATTR_DISPLAY_POS] =
                    sg::VertexAttrState { format: sg::VertexFormat::Float3, ..Default::default() };
                attrs[shader::ATTR_DISPLAY_COLOR0] =
                    sg::VertexAttrState { format: sg::VertexFormat::Float4, ..Default::default() };
                attrs
            },
            ..Default::default()
        },
        index_type: sg::IndexType::Uint16,
        cull_mode: sg::CullMode::Back,
        depth: sg::DepthState {
            compare: sg::CompareFunc::LessEqual,
            write_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    // one-time init of particle velocities
    let pip = sg::make_pipeline(&sg::PipelineDesc {
        compute: true,
        shader: sg::make_shader(&shader::init_shader_desc(sg::query_backend())),
        ..Default::default()
    });
    sg::begin_pass(&sg::Pass { compute: true, ..Default::default() });
    sg::apply_pipeline(pip);
    sg::apply_bindings(&state.compute.bind);
    sg::dispatch(MAX_PARTICLES / 64, 1, 1);
    sg::end_pass();
    sg::destroy_pipeline(pip);
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    if !sg::query_features().compute {
        draw_fallback(state);
        return;
    }

    state.num_particles += NUM_PARTICLES_EMITTED_PER_FRAME;
    if state.num_particles > MAX_PARTICLES {
        state.num_particles = MAX_PARTICLES;
    }
    let dt = sapp::frame_duration() as f32;

    // compute pass to update particle positions
    let cs_params = shader::CsParams { dt, num_particles: state.num_particles as i32, _pad_8: [0; 8] };
    sg::begin_pass(&sg::Pass { compute: true, ..Default::default() });
    sg::apply_pipeline(state.compute.pip);
    sg::apply_bindings(&state.compute.bind);
    sg::apply_uniforms(shader::UB_CS_PARAMS, &sg::value_as_range(&cs_params));
    sg::dispatch((state.num_particles + 63) / 64, 1, 1);
    sg::end_pass();

    // render pass to render the particles via instancing, with the
    // instance positions coming from the storage buffer
    state.ry += 60.0 * dt;
    let vs_params = compute_vs_params(state);
    sg::begin_pass(&sg::Pass {
        action: state.display.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sg::apply_pipeline(state.display.pip);
    sg::apply_bindings(&state.display.bind);
    sg::apply_uniforms(shader::UB_VS_PARAMS, &sg::value_as_range(&vs_params));
    sg::draw(0, 24, state.num_particles);
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sg::shutdown();
    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn draw_fallback(state: &State) {
    sg::begin_pass(&sg::Pass {
        action: state.display.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sg::end_pass();
    sg::commit();
}

fn compute_vs_params(state: &State) -> shader::VsParams {
    let proj = m::persp_mat4(60.0, sapp::widthf() / sapp::heightf(), 0.01, 50.0);
    let view = m::lookat_mat4(m::vec3(0.0, 1.5, 12.0), m::Vec3::ZERO, m::Vec3::UP);
    let view_proj = m::mul_mat4(proj, view);
    shader::VsParams { mvp: m::mul_mat4(view_proj, m::rotate_mat4(state.ry, m::vec3(0.0, 1.0, 0.0))) }
}

fn main() {
    let state = Box::new(State {
        num_particles: 0,
        ry: 0.0,
        compute: ComputeState { pip: sg::Pipeline::new(), bind: sg::Bindings::new() },
        display: DisplayState {
            pip: sg::Pipeline::new(),
            bind: sg::Bindings::new(),
            pass_action: sg::PassAction::new(),
        },
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
        window_title: c"instancing-compute.rs".as_ptr(),
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        logger: sapp::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
}
