//------------------------------------------------------------------------------
//  sgl-context/sgl-context.rs
//
//  Demonstrates how to render in different render passes with sokol/gl
//  using sokol/gl contexts.
//------------------------------------------------------------------------------

use sokol::{app as sapp, gfx as sg, gl as sgl, glue as sglue, log as slog};

const OFFSCREEN_PIXELFORMAT: sg::PixelFormat = sg::PixelFormat::Rgba8;
const OFFSCREEN_SAMPLECOUNT: i32 = 1;
const OFFSCREEN_WIDTH: i32 = 32;
const OFFSCREEN_HEIGHT: i32 = 32;

struct Offscreen {
    pass_action: sg::PassAction,
    attachments: sg::Attachments,
    img: sg::Image,
    sgl_context: sgl::Context,
}

struct Display {
    pass_action: sg::PassAction,
    smp: sg::Sampler,
    sgl_pip: sgl::Pipeline,
}

struct State {
    offscreen: Offscreen,
    display: Display,
}

static mut STATE: State = State {
    offscreen: Offscreen {
        pass_action: sg::PassAction::new(),
        attachments: sg::Attachments::new(),
        img: sg::Image::new(),
        sgl_context: sgl::Context::new(),
    },
    display: Display {
        pass_action: sg::PassAction::new(),
        smp: sg::Sampler::new(),
        sgl_pip: sgl::Pipeline::new(),
    },
};

extern "C" fn init() {
    let state = unsafe { &mut STATE };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });

    sgl::setup(&sgl::Desc {
        max_vertices: 64,
        max_commands: 16,
        logger: sgl::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });

    state.display.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.5, g: 0.7, b: 1.0, a: 1.0 },
        ..Default::default()
    };
    state.display.sgl_pip = sgl::context_make_pipeline(sgl::default_context(), &sg::PipelineDesc {
        cull_mode: sg::CullMode::Back,
        depth: sg::DepthState {
            write_enabled: true,
            compare: sg::CompareFunc::LessEqual,
            ..Default::default()
        },
        ..Default::default()
    });

    // create a sokol-gl context compatible with the offscreen render pass
    // (specific color pixel format, no depth-stencil-surface, no MSAA)
    state.offscreen.sgl_context = sgl::make_context(&sgl::ContextDesc {
        max_vertices: 8,
        max_commands: 4,
        color_format: OFFSCREEN_PIXELFORMAT,
        depth_format: sg::PixelFormat::None,
        sample_count: OFFSCREEN_SAMPLECOUNT,
        ..Default::default()
    });

    // create an offscreen render target texture, pass, and pass_action
    state.offscreen.img = sg::make_image(&sg::ImageDesc {
        render_target: true,
        width: OFFSCREEN_WIDTH,
        height: OFFSCREEN_HEIGHT,
        pixel_format: OFFSCREEN_PIXELFORMAT,
        sample_count: OFFSCREEN_SAMPLECOUNT,
        ..Default::default()
    });
    let mut atts_desc = sg::AttachmentsDesc::new();
    atts_desc.colors[0] = sg::AttachmentDesc { image: state.offscreen.img, ..Default::default() };
    state.offscreen.attachments = sg::make_attachments(&atts_desc);
    state.offscreen.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };

    // a sampler to sample the offscreen render target as texture
    state.display.smp = sg::make_sampler(&sg::SamplerDesc {
        wrap_u: sg::Wrap::ClampToEdge,
        wrap_v: sg::Wrap::ClampToEdge,
        min_filter: sg::Filter::Nearest,
        mag_filter: sg::Filter::Nearest,
        ..Default::default()
    });
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

    let t = (sapp::frame_duration() * 60.0) as f32;
    let a = ((sapp::frame_count() as f32) * t).to_radians();

    // draw a rotating quad into the offscreen render target texture
    sgl::set_context(state.offscreen.sgl_context);
    sgl::defaults();
    sgl::matrix_mode_modelview();
    sgl::rotate(a, 0.0, 0.0, 1.0);
    draw_quad();

    // draw a rotating 3D cube, using the offscreen render target as texture
    sgl::set_context(sgl::default_context());
    sgl::defaults();
    sgl::enable_texture();
    sgl::texture(state.offscreen.img, state.display.smp);
    sgl::load_pipeline(state.display.sgl_pip);
    sgl::matrix_mode_projection();
    sgl::perspective(f32::to_radians(45.0), sapp::widthf() / sapp::heightf(), 0.1, 100.0);
    let eye = [f32::sin(a) * 6.0, f32::sin(a) * 3.0, f32::cos(a) * 6.0];
    sgl::matrix_mode_modelview();
    sgl::lookat(eye[0], eye[1], eye[2], 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    draw_cube();

    // do the actual offscreen and display rendering in sokol-gfx passes
    sg::begin_pass(&sg::Pass {
        action: state.offscreen.pass_action,
        attachments: state.offscreen.attachments,
        ..Default::default()
    });
    sgl::context_draw(state.offscreen.sgl_context);
    sg::end_pass();
    sg::begin_pass(&sg::Pass {
        action: state.display.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sgl::context_draw(sgl::default_context());
    sg::end_pass();
    sg::commit();
}

// helper function to draw a colored quad with sokol-gl
#[rustfmt::skip]
fn draw_quad() {
    sgl::begin_quads();
    sgl::v2f_c3b( 0.0, -1.0,  255, 000, 000);
    sgl::v2f_c3b( 1.0,  0.0,  000, 000, 255);
    sgl::v2f_c3b( 0.0,  1.0,  000, 255, 255);
    sgl::v2f_c3b(-1.0,  0.0,  000, 255, 000);
    sgl::end();
}

// helper function to draw a textured cube with sokol-gl
#[rustfmt::skip]
fn draw_cube() {
    sgl::begin_quads();
    sgl::v3f_t2f(-1.0,  1.0, -1.0,   0.0, 1.0);
    sgl::v3f_t2f( 1.0,  1.0, -1.0,   1.0, 1.0);
    sgl::v3f_t2f( 1.0, -1.0, -1.0,   1.0, 0.0);
    sgl::v3f_t2f(-1.0, -1.0, -1.0,   0.0, 0.0);
    sgl::v3f_t2f(-1.0, -1.0,  1.0,   0.0, 1.0);
    sgl::v3f_t2f( 1.0, -1.0,  1.0,   1.0, 1.0);
    sgl::v3f_t2f( 1.0,  1.0,  1.0,   1.0, 0.0);
    sgl::v3f_t2f(-1.0,  1.0,  1.0,   0.0, 0.0);
    sgl::v3f_t2f(-1.0, -1.0,  1.0,   0.0, 1.0);
    sgl::v3f_t2f(-1.0,  1.0,  1.0,   1.0, 1.0);
    sgl::v3f_t2f(-1.0,  1.0, -1.0,   1.0, 0.0);
    sgl::v3f_t2f(-1.0, -1.0, -1.0,   0.0, 0.0);
    sgl::v3f_t2f( 1.0, -1.0,  1.0,   0.0, 1.0);
    sgl::v3f_t2f( 1.0, -1.0, -1.0,   1.0, 1.0);
    sgl::v3f_t2f( 1.0,  1.0, -1.0,   1.0, 0.0);
    sgl::v3f_t2f( 1.0,  1.0,  1.0,   0.0, 0.0);
    sgl::v3f_t2f( 1.0, -1.0, -1.0,   0.0, 1.0);
    sgl::v3f_t2f( 1.0, -1.0,  1.0,   1.0, 1.0);
    sgl::v3f_t2f(-1.0, -1.0,  1.0,   1.0, 0.0);
    sgl::v3f_t2f(-1.0, -1.0, -1.0,   0.0, 0.0);
    sgl::v3f_t2f(-1.0,  1.0, -1.0,   0.0, 1.0);
    sgl::v3f_t2f(-1.0,  1.0,  1.0,   1.0, 1.0);
    sgl::v3f_t2f( 1.0,  1.0,  1.0,   1.0, 0.0);
    sgl::v3f_t2f( 1.0,  1.0, -1.0,   0.0, 0.0);
    sgl::end();
}

extern "C" fn cleanup() {
    sgl::shutdown();
    sg::shutdown();
}

fn main() {
    let window_title = b"sgl-context\0".as_ptr() as *const _;

    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        frame_cb: Some(frame),
        cleanup_cb: Some(cleanup),
        width: 800,
        height: 600,
        sample_count: 4,
        window_title,
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        ..Default::default()
    })
}
