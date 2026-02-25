//------------------------------------------------------------------------------
//  offscreen/offscreen.rust
//
//  Render to an offscreen rendertarget texture, and use this texture
//  for rendering to the display.
//------------------------------------------------------------------------------

mod math;
mod shader;

use std::ffi;

use sokol::{
    app as sapp,
    gfx::{self as sg, PixelFormat},
    glue as sglue, shape as sshape,
};

struct OffscreenState {
    pass: sg::Pass,
    pip: sg::Pipeline,
    bind: sg::Bindings,
}

struct DisplayState {
    pass_action: sg::PassAction,
    pip: sg::Pipeline,
    bind: sg::Bindings,
}

struct State {
    donut: sshape::ElementRange,
    sphere: sshape::ElementRange,
    rx: f32,
    ry: f32,
    offscreen: OffscreenState,
    display: DisplayState,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // default pass action: clear to blue-ish
    state.display.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.25, g: 0.45, b: 0.65, a: 1.0 },
        ..Default::default()
    };

    // offscreen pass action: clear to grey
    state.offscreen.pass.action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.25, g: 0.25, b: 0.25, a: 1.0 },
        ..Default::default()
    };

    // setup the color- and depth-stencil-attachment images and views
    let offscreen_width = 256;
    let offscreen_height = 256;
    let offscreen_sample_count = 1;
    let color_img = sg::make_image(&sg::ImageDesc {
        usage: sg::ImageUsage { color_attachment: true, ..Default::default() },
        width: offscreen_width,
        height: offscreen_height,
        sample_count: offscreen_sample_count,
        pixel_format: sg::PixelFormat::Rgba8,
        ..Default::default()
    });
    let depth_img = sg::make_image(&sg::ImageDesc {
        usage: sg::ImageUsage { depth_stencil_attachment: true, ..Default::default() },
        width: offscreen_width,
        height: offscreen_height,
        sample_count: offscreen_sample_count,
        pixel_format: sg::PixelFormat::Depth,
        ..Default::default()
    });

    // the offscreen render pass needs a color- and depth-stencil-attachment view
    state.offscreen.pass.attachments.colors[0] = sg::make_view(&sg::ViewDesc {
        color_attachment: sg::ImageViewDesc { image: color_img, ..Default::default() },
        ..Default::default()
    });
    state.offscreen.pass.attachments.depth_stencil = sg::make_view(&sg::ViewDesc {
        depth_stencil_attachment: sg::ImageViewDesc { image: depth_img, ..Default::default() },
        ..Default::default()
    });

    // the display render pass needs a texture view on the color image
    state.display.bind.views[shader::VIEW_TEX] = sg::make_view(&sg::ViewDesc {
        texture: sg::TextureViewDesc { image: color_img, ..Default::default() },
        ..Default::default()
    });

    // a donut shape which is rendered into the offscreen render target, and
    // a sphere shape which is rendered into the default framebuffer
    let vertices = [sshape::Vertex::new(); 4_000];
    let vertices = sshape::value_as_range(&vertices);
    let indices = [0u16; 24_000];
    let indices = sshape::value_as_range(&indices);
    let buf = sshape::Buffer {
        vertices: sshape::BufferItem { buffer: vertices, ..Default::default() },
        indices: sshape::BufferItem { buffer: indices, ..Default::default() },
        ..Default::default()
    };
    let buf = sshape::build_torus(
        &buf,
        &sshape::Torus { radius: 0.5, ring_radius: 0.3, sides: 20, rings: 36, ..Default::default() },
    );
    state.donut = sshape::element_range(&buf);
    let buf = sshape::build_sphere(
        &buf,
        &sshape::Sphere { radius: 0.5, slices: 72, stacks: 40, ..Default::default() },
    );
    state.sphere = sshape::element_range(&buf);

    let vbuf = sg::make_buffer(&sshape::vertex_buffer_desc(&buf));
    state.offscreen.bind.vertex_buffers[0] = vbuf;
    state.display.bind.vertex_buffers[0] = vbuf;

    let ibuf = sg::make_buffer(&sshape::index_buffer_desc(&buf));
    state.offscreen.bind.index_buffer = ibuf;
    state.display.bind.index_buffer = ibuf;

    // shader and pipeline object for offscreen rendering
    state.offscreen.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::offscreen_shader_desc(sg::query_backend())),
        layout: {
            let mut l = sg::VertexLayoutState::new();
            l.buffers[0] = sshape::vertex_buffer_layout_state();
            l.attrs[shader::ATTR_OFFSCREEN_POSITION] = sshape::position_vertex_attr_state();
            l.attrs[shader::ATTR_OFFSCREEN_NORMAL] = sshape::normal_vertex_attr_state();
            l
        },
        index_type: sg::IndexType::Uint16,
        cull_mode: sg::CullMode::Back,
        sample_count: offscreen_sample_count,
        depth: sg::DepthState {
            pixel_format: sg::PixelFormat::Depth,
            compare: sg::CompareFunc::LessEqual,
            write_enabled: true,
            ..Default::default()
        },
        colors: {
            let mut c = [sg::ColorTargetState::new(); sg::MAX_COLOR_ATTACHMENTS];
            c[0].pixel_format = PixelFormat::Rgba8;
            c
        },
        ..Default::default()
    });

    // shader and pipeline object for the default render pass
    state.display.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: sg::make_shader(&shader::default_shader_desc(sg::query_backend())),
        layout: {
            let mut l = sg::VertexLayoutState::new();
            l.buffers[0] = sshape::vertex_buffer_layout_state();
            l.attrs[shader::ATTR_DEFAULT_POSITION] = sshape::position_vertex_attr_state();
            l.attrs[shader::ATTR_DEFAULT_NORMAL] = sshape::normal_vertex_attr_state();
            l.attrs[shader::ATTR_DEFAULT_TEXCOORD0] = sshape::texcoord_vertex_attr_state();
            l
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

    // a sampler object for sampling the render target texture
    state.display.bind.samplers[shader::SMP_SMP] = sg::make_sampler(&sg::SamplerDesc {
        min_filter: sg::Filter::Linear,
        mag_filter: sg::Filter::Linear,
        wrap_u: sg::Wrap::Repeat,
        wrap_v: sg::Wrap::Repeat,
        ..Default::default()
    });
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    // Cast user data to a borrowed state reference
    let state = unsafe { &mut *(user_data as *mut State) };

    let dt = sapp::frame_duration() as f32 * 60.0;
    state.rx += 1.0 * dt;
    state.ry += 2.0 * dt;
    let aspect = sapp::widthf() / sapp::heightf();

    // the offscreen pass, rendering a rotating untextured donut into a render target image
    sg::begin_pass(&state.offscreen.pass);
    sg::apply_pipeline(state.offscreen.pip);
    sg::apply_bindings(&state.offscreen.bind);
    sg::apply_uniforms(
        shader::UB_VS_PARAMS,
        &sg::value_as_range(&compute_vs_params(state.rx, state.ry, 1.0, 2.5)),
    );
    sg::draw(state.donut.base_element as usize, state.donut.num_elements as usize, 1);
    sg::end_pass();

    // and the display pass, rendering a rotating textured sphere, using the previously
    // rendered offscreen render target as texture
    sg::begin_pass(&sg::Pass {
        action: state.display.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sg::apply_pipeline(state.display.pip);
    sg::apply_bindings(&state.display.bind);
    sg::apply_uniforms(
        shader::UB_VS_PARAMS,
        &sg::value_as_range(&compute_vs_params(-state.rx * 0.25, state.ry * 0.25, aspect, 2.0)),
    );
    sg::draw(state.sphere.base_element as usize, state.sphere.num_elements as usize, 1);
    sg::end_pass();

    sg::commit();
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sg::shutdown();

    // Convert the user_data back to an owned ptr so that it gets dropped correctly
    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    let state = Box::new(State {
        donut: sshape::ElementRange::new(),
        sphere: sshape::ElementRange::new(),
        rx: 0.0,
        ry: 0.0,
        offscreen: OffscreenState {
            pass: sg::Pass::new(),
            pip: sg::Pipeline::new(),
            bind: sg::Bindings::new(),
        },
        display: DisplayState {
            pass_action: sg::PassAction::new(),
            pip: sg::Pipeline::new(),
            bind: sg::Bindings::new(),
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
        window_title: c"offscreen.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },

        ..Default::default()
    });
}

fn compute_vs_params(rx: f32, ry: f32, aspect: f32, eye_dist: f32) -> shader::VsParams {
    let proj = math::persp_mat4(45.0, aspect, 0.01, 10.0);
    let view =
        math::lookat_mat4(math::Vec3 { x: 0.0, y: 0.0, z: eye_dist }, math::Vec3::ZERO, math::Vec3::UP);
    let view_proj = math::mul_mat4(proj, view);
    let rxm = math::rotate_mat4(rx, math::Vec3 { x: 1.0, y: 0.0, z: 0.0 });
    let rym = math::rotate_mat4(ry, math::Vec3 { x: 0.0, y: 1.0, z: 0.0 });
    let model = math::mul_mat4(rxm, rym);
    shader::VsParams { mvp: math::mul_mat4(view_proj, model) }
}
