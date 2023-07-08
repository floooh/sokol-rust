//------------------------------------------------------------------------------
//  instancing.rs
//
//  Demonstrate simple hardware-instancing using a static geometry buffer
//  and a dynamic instance-data buffer.
//------------------------------------------------------------------------------

mod math;
mod shader;

use math as m;
use sokol::{app as sapp, gfx as sg, glue as sglue, log as slog};

pub const MAX_PARTICLES: usize = 512 * 1024;
pub const NUM_PARTICLES_EMITTED_PER_FRAME: usize = 10;

struct State {
    pub pass_action: sg::PassAction,
    pub pip: sg::Pipeline,
    pub bind: sg::Bindings,
    pub ry: f32,
    pub cur_num_particles: usize,
    pub pos: [m::Vec3; MAX_PARTICLES],
    pub vel: [m::Vec3; MAX_PARTICLES],
}

static mut STATE: State = State {
    pass_action: sg::PassAction::new(),
    pip: sg::Pipeline::new(),
    bind: sg::Bindings::new(),
    ry: 0.0,
    cur_num_particles: 0,
    pos: [m::Vec3::ZERO; MAX_PARTICLES],
    vel: [m::Vec3::ZERO; MAX_PARTICLES],
};

extern "C" fn init() {
    let state = unsafe { &mut STATE };
    sg::setup(&sg::Desc {
        context: sglue::context(),
        logger: sg::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });

    // a pass action for the default render pass
    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };

    // vertex buffer for static geometry, goes into vertex-buffer-slot 0
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
    state.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(vertices),
        _type: sg::BufferType::Vertexbuffer,
        ..Default::default()
    });

    // index buffer for static geometry
    #[rustfmt::skip]
    let indices: &[u16] = &[
        0, 1, 2,    0, 2, 3,    0, 3, 4,    0, 4, 1,
        5, 1, 2,    5, 2, 3,    5, 3, 4,    5, 4, 1,
    ];
    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        _type: sg::BufferType::Indexbuffer,
        data: sg::slice_as_range(indices),
        ..Default::default()
    });

    // empty, dynamic instance-data vertex buffer, goes into vertex-buffer-slot 1
    state.bind.vertex_buffers[1] = sg::make_buffer(&sg::BufferDesc {
        _type: sg::BufferType::Vertexbuffer,
        size: MAX_PARTICLES * std::mem::size_of::<m::Vec3>(),
        usage: sg::Usage::Stream,
        ..Default::default()
    });

    // a shader and pipeline object
    #[rustfmt::skip]
    let pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::instancing_shader_desc(sg::query_backend())),
        layout: sg::VertexLayoutState {
            buffers: {
                let mut buffers = [sg::VertexBufferLayoutState::new(); sg::MAX_VERTEX_BUFFERS];

                // vertex buffer at slot 1 must step per instance
                buffers[1] = sg::VertexBufferLayoutState { step_func: sg::VertexStep::PerInstance, ..Default::default() };

                buffers
            },
            attrs: {
                let mut attrs = [sg::VertexAttrState::new(); sg::MAX_VERTEX_ATTRIBUTES];

                attrs[shader::ATTR_VS_POS     ] = sg::VertexAttrState { format: sg::VertexFormat::Float3, buffer_index: 0, ..Default::default() };
                attrs[shader::ATTR_VS_COLOR0  ] = sg::VertexAttrState { format: sg::VertexFormat::Float4, buffer_index: 0, ..Default::default() };
                attrs[shader::ATTR_VS_INST_POS] = sg::VertexAttrState { format: sg::VertexFormat::Float3, buffer_index: 1, ..Default::default() };

                attrs
            },
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
    state.pip = pip;
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

    let frame_time = sapp::frame_duration() as f32;

    // emit new particles
    for _ in 0..NUM_PARTICLES_EMITTED_PER_FRAME {
        if state.cur_num_particles < MAX_PARTICLES {
            state.vel[state.cur_num_particles] = m::vec3(rand(-0.5, 0.5), rand(2.0, 2.5), rand(-0.5, 0.5));
            state.cur_num_particles += 1;
        } else {
            break;
        }
    }

    // update particle positions
    for i in 0..state.cur_num_particles {
        state.vel[i].y -= 1.0 * frame_time;
        state.pos[i] = state.pos[i] + state.vel[i] * frame_time;

        // bounce back from ground
        if state.pos[i].y < -2.0 {
            state.pos[i].y = -1.8;
            state.vel[i].y = -state.vel[i].y;
            state.vel[i] = state.vel[i] * 0.8;
        }
    }

    // update instance data
    sg::update_buffer(state.bind.vertex_buffers[1], &sg::slice_as_range(&state.pos));

    // vertex shader uniform data with model-view-projection matrix
    let proj = m::persp_mat4(60.0, sapp::widthf() / sapp::heightf(), 0.01, 50.0);
    let view = m::lookat_mat4(m::vec3(0.0, 1.5, 12.0), m::Vec3::ZERO, m::Vec3::UP);
    let view_proj = m::mul_mat4(proj, view);
    state.ry += 60.0 * frame_time;
    let vs_params =
        shader::VsParams { mvp: m::mul_mat4(view_proj, m::rotate_mat4(state.ry, m::vec3(0.0, 1.0, 0.0))) };

    // ...and draw
    sg::begin_default_pass(&state.pass_action, sapp::width(), sapp::height());
    sg::apply_pipeline(state.pip);
    sg::apply_bindings(&state.bind);
    sg::apply_uniforms(sg::ShaderStage::Vs, shader::SLOT_VS_PARAMS, &sg::value_as_range(&vs_params));
    sg::draw(0, 24, state.cur_num_particles);
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    sg::shutdown()
}

fn main() {
    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        frame_cb: Some(frame),
        cleanup_cb: Some(cleanup),
        width: 800,
        height: 600,
        sample_count: 4,
        window_title: b"instancing\0".as_ptr() as _,
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        logger: sapp::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
}

fn xorshift32() -> u32 {
    static mut X: u32 = 0x12345678;

    unsafe {
        X ^= X << 13;
        X ^= X >> 17;
        X ^= X << 5;

        X
    }
}

fn rand(min_val: f32, max_val: f32) -> f32 {
    let r = xorshift32();
    ((r & 0xFFFF) as f32 / 0x10000 as f32) * (max_val - min_val) + min_val
}
