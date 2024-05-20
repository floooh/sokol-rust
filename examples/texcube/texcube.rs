//------------------------------------------------------------------------------
//  texcube.rs
//  Texture creation, rendering with texture, packed vertex components.
//------------------------------------------------------------------------------

mod math;
mod shader;

use math as m;
use sokol::{app as sapp, gfx as sg, glue as sglue, log as slog};
use std::ffi;

struct State {
    pub pass_action: sg::PassAction,
    pub pip: sg::Pipeline,
    pub bind: sg::Bindings,
    pub rx: f32,
    pub ry: f32,
}

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub color: u32,
    pub u: u16,
    pub v: u16,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });

    /*
        Cube vertex buffer with packed vertex formats for color and texture coords.
        Note that a vertex format which must be portable across all
        backends must only use the normalized integer formats
        (BYTE4N, UBYTE4N, SHORT2N, SHORT4N), which can be converted
        to floating point formats in the vertex shader inputs.
        The reason is that D3D11 cannot convert from non-normalized
        formats to floating point inputs (only to integer inputs),
        and WebGL2 / GLES2 don't support integer vertex shader inputs.
    */
    #[rustfmt::skip]
    const VERTICES: &[Vertex] = &[
        // pos                                color              uvs
        Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFF0000FF, u:     0, v:     0 },
        Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFF0000FF, u: 32767, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFF0000FF, u: 32767, v: 32767 },
        Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFF0000FF, u:     0, v: 32767 },

        Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFF00FF00, u:     0, v:     0 },
        Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFF00FF00, u: 32767, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFF00FF00, u: 32767, v: 32767 },
        Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFF00FF00, u:     0, v: 32767 },

        Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFFFF0000, u:     0, v:     0 },
        Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFFFF0000, u: 32767, v:     0 },
        Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFFFF0000, u: 32767, v: 32767 },
        Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFFFF0000, u:     0, v: 32767 },

        Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFFFF007F, u:     0, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFFFF007F, u: 32767, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFFFF007F, u: 32767, v: 32767 },
        Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFFFF007F, u:     0, v: 32767 },

        Vertex { x: -1.0,  y: -1.0, z: -1.0,  color: 0xFFFF7F00, u:     0, v:     0 },
        Vertex { x: -1.0,  y: -1.0, z:  1.0,  color: 0xFFFF7F00, u: 32767, v:     0 },
        Vertex { x:  1.0,  y: -1.0, z:  1.0,  color: 0xFFFF7F00, u: 32767, v: 32767 },
        Vertex { x:  1.0,  y: -1.0, z: -1.0,  color: 0xFFFF7F00, u:     0, v: 32767 },

        Vertex { x: -1.0,  y:  1.0, z: -1.0,  color: 0xFF007FFF, u:     0, v:     0 },
        Vertex { x: -1.0,  y:  1.0, z:  1.0,  color: 0xFF007FFF, u: 32767, v:     0 },
        Vertex { x:  1.0,  y:  1.0, z:  1.0,  color: 0xFF007FFF, u: 32767, v: 32767 },
        Vertex { x:  1.0,  y:  1.0, z: -1.0,  color: 0xFF007FFF, u:     0, v: 32767 },
    ];

    state.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(VERTICES),
        _type: sg::BufferType::Vertexbuffer,
        ..Default::default()
    });

    // create an index buffer for the cube
    #[rustfmt::skip]
    const INDICES: &[u16] = &[
        0, 1, 2,  0, 2, 3,
        6, 5, 4,  7, 6, 4,
        8, 9, 10,  8, 10, 11,
        14, 13, 12,  15, 14, 12,
        16, 17, 18,  16, 18, 19,
        22, 21, 20,  23, 22, 20,
    ];
    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(INDICES),
        _type: sg::BufferType::Indexbuffer,
        ..Default::default()
    });

    // create a checkerboard texture
    let pixels: [u32; 4 * 4] = [
        0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF, 0xFF000000, 0xFF000000, 0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF, 0xFF000000, 0xFF000000, 0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF,
    ];
    // NOTE: SLOT_tex is provided by shader code generation
    let mut image_desc = sg::ImageDesc { width: 4, height: 4, ..Default::default() };
    image_desc.data.subimage[0][0] = sg::slice_as_range(&pixels);
    state.bind.fs.images[shader::SLOT_TEX] = sg::make_image(&image_desc);

    // create a sampler object
    state.bind.fs.samplers[shader::SLOT_SMP] = sg::make_sampler(&sg::SamplerDesc {
        min_filter: sg::Filter::Nearest,
        mag_filter: sg::Filter::Nearest,
        wrap_u: sg::Wrap::Repeat,
        wrap_v: sg::Wrap::Repeat,
        ..Default::default()
    });

    // shader and pipeline object
    #[rustfmt::skip]
    let pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::texcube_shader_desc(sg::query_backend())),
        layout: sg::VertexLayoutState {
            attrs: {
                let mut attrs = [sg::VertexAttrState::new(); sg::MAX_VERTEX_ATTRIBUTES];

                attrs[shader::ATTR_VS_POS] = sg::VertexAttrState { format: sg::VertexFormat::Float3, ..Default::default() };
                attrs[shader::ATTR_VS_COLOR0] = sg::VertexAttrState { format: sg::VertexFormat::Ubyte4n, ..Default::default() };
                attrs[shader::ATTR_VS_TEXCOORD0] = sg::VertexAttrState { format: sg::VertexFormat::Short2n, ..Default::default() };

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
    state.pip = pip;

    // default pass action, clear to blue-ish
    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.25, g: 0.5, b: 0.75, a: 1.0 },
        ..Default::default()
    };
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

    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    let state = Box::new(State {
        pass_action: sg::PassAction::new(),
        pip: sg::Pipeline::new(),
        bind: sg::Bindings::new(),
        rx: 0.0,
        ry: 0.0,
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
        window_title: c"texcube.rs".as_ptr(),
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        logger: sapp::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
}
