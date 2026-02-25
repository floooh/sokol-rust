use std::ffi;

use sokol::{app as sapp, gfx as sg, gl as sgl, glue as sglue};

#[derive(Default)]
struct State {
    pass_action: sg::PassAction,
    tex_view: sg::View,
    smp: sg::Sampler,
    pip3d: sgl::Pipeline,
    quad: Quad,
    cube: Cube,
    texcube: Texcube,
}

#[derive(Default)]
struct Quad {
    rot: f32,
}

#[derive(Default)]
struct Cube {
    rot_x: f32,
    rot_y: f32,
}

#[derive(Default)]
struct Texcube {
    time_accum: f32,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    // setup sokol-gfx
    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });
    // setup sokol-gl
    sgl::setup(&sgl::Desc {
        logger: sgl::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // a checkerboard image and texture view
    let img_width: usize = 8;
    let img_height: usize = 8;
    let img = sg::make_image(&sg::ImageDesc {
        width: img_width as i32,
        height: img_height as i32,
        data: {
            let mut data = sg::ImageData::new();
            let mut res = [0u32; 64];
            for y in 0..img_height {
                for x in 0..img_width {
                    res[y * img_width + x] = if 0 == (y ^ x) & 1 { 0xFF_00_00_00 } else { 0xFF_FF_FF_FF };
                }
            }
            data.mip_levels[0] = sg::value_as_range(&res);
            data
        },
        ..Default::default()
    });
    state.tex_view = sg::make_view(&sg::ViewDesc {
        texture: sg::TextureViewDesc { image: img, ..Default::default() },
        ..Default::default()
    });

    // ...and a sampler
    state.smp = sg::make_sampler(&sg::SamplerDesc {
        min_filter: sg::Filter::Nearest,
        mag_filter: sg::Filter::Nearest,
        ..Default::default()
    });

    // create a pipeline object for 3d rendering, with less-equal
    // depth-test and cull-face enabled, note that we don't provide
    // a shader, vertex-layout, pixel formats and sample count here,
    // these are all filled in by sokol-gl
    state.pip3d = sgl::make_pipeline(&sg::PipelineDesc {
        depth: sg::DepthState {
            write_enabled: true,
            compare: sg::CompareFunc::LessEqual,
            ..Default::default()
        },
        cull_mode: sg::CullMode::Back,
        ..Default::default()
    });

    // pass-action to clear to black
    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    // frame time 'normalized' to 60fps
    let dt = sapp::frame_duration() * 60.0;

    // compute viewport rectangles so that the views are horizontally
    // centered and keep a 1:1 aspect ratio
    let dw = sapp::widthf();
    let dh = sapp::heightf();
    let ww = dh * 0.5;
    let hh = dh * 0.5;
    let x0 = dw * 0.5 - hh;
    let x1 = dw * 0.5;
    let y0 = 0.0;
    let y1 = dh * 0.5;

    sgl::viewportf(x0, y0, ww, hh, true);
    draw_triangle();
    sgl::viewportf(x1, y0, ww, hh, true);
    draw_quad(state, dt);
    sgl::viewportf(x0, y1, ww, hh, true);
    draw_cubes(state, dt);
    sgl::viewportf(x1, y1, ww, hh, true);
    draw_tex_cube(state, dt);
    sgl::viewportf(0.0, 0.0, dw, dh, true);

    sg::begin_pass(&sg::Pass {
        action: state.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sgl::draw();
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sgl::shutdown();
    sg::shutdown();

    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn draw_triangle() {
    sgl::defaults();
    sgl::begin_triangles();
    sgl::v2f_c3b(0.0, 0.5, 255, 0, 0);
    sgl::v2f_c3b(-0.5, -0.5, 0, 0, 255);
    sgl::v2f_c3b(0.5, -0.5, 0, 255, 0);
    sgl::end();
}

fn draw_quad(state: &mut State, dt: f64) {
    state.quad.rot += 1.0 * dt as f32;
    let scale: f32 = 1.0 + sgl::rad(state.quad.rot).sin() * 0.5;
    sgl::defaults();
    sgl::rotate(sgl::rad(state.quad.rot), 0.0, 0.0, 1.0);
    sgl::scale(scale, scale, 1.0);
    sgl::begin_quads();
    sgl::v2f_c3b(-0.5, -0.5, 255, 255, 0);
    sgl::v2f_c3b(0.5, -0.5, 0, 255, 0);
    sgl::v2f_c3b(0.5, 0.5, 0, 0, 255);
    sgl::v2f_c3b(-0.5, 0.5, 255, 0, 0);
    sgl::end();
}

// vertex specification for a cube with colored sides and texture coords
fn draw_cube() {
    sgl::begin_quads();
    sgl::c3f(1.0, 0.0, 0.0);
    sgl::v3f_t2f(-1.0, 1.0, -1.0, -1.0, 1.0);
    sgl::v3f_t2f(1.0, 1.0, -1.0, 1.0, 1.0);
    sgl::v3f_t2f(1.0, -1.0, -1.0, 1.0, -1.0);
    sgl::v3f_t2f(-1.0, -1.0, -1.0, -1.0, -1.0);
    sgl::c3f(0.0, 1.0, 0.0);
    sgl::v3f_t2f(-1.0, -1.0, 1.0, -1.0, 1.0);
    sgl::v3f_t2f(1.0, -1.0, 1.0, 1.0, 1.0);
    sgl::v3f_t2f(1.0, 1.0, 1.0, 1.0, -1.0);
    sgl::v3f_t2f(-1.0, 1.0, 1.0, -1.0, -1.0);
    sgl::c3f(0.0, 0.0, 1.0);
    sgl::v3f_t2f(-1.0, -1.0, 1.0, -1.0, 1.0);
    sgl::v3f_t2f(-1.0, 1.0, 1.0, 1.0, 1.0);
    sgl::v3f_t2f(-1.0, 1.0, -1.0, 1.0, -1.0);
    sgl::v3f_t2f(-1.0, -1.0, -1.0, -1.0, -1.0);
    sgl::c3f(1.0, 0.5, 0.0);
    sgl::v3f_t2f(1.0, -1.0, 1.0, -1.0, 1.0);
    sgl::v3f_t2f(1.0, -1.0, -1.0, 1.0, 1.0);
    sgl::v3f_t2f(1.0, 1.0, -1.0, 1.0, -1.0);
    sgl::v3f_t2f(1.0, 1.0, 1.0, -1.0, -1.0);
    sgl::c3f(0.0, 0.5, 1.0);
    sgl::v3f_t2f(1.0, -1.0, -1.0, -1.0, 1.0);
    sgl::v3f_t2f(1.0, -1.0, 1.0, 1.0, 1.0);
    sgl::v3f_t2f(-1.0, -1.0, 1.0, 1.0, -1.0);
    sgl::v3f_t2f(-1.0, -1.0, -1.0, -1.0, -1.0);
    sgl::c3f(1.0, 0.0, 0.5);
    sgl::v3f_t2f(-1.0, 1.0, -1.0, -1.0, 1.0);
    sgl::v3f_t2f(-1.0, 1.0, 1.0, 1.0, 1.0);
    sgl::v3f_t2f(1.0, 1.0, 1.0, 1.0, -1.0);
    sgl::v3f_t2f(1.0, 1.0, -1.0, -1.0, -1.0);
    sgl::end();
}

fn draw_cubes(state: &mut State, dt: f64) {
    state.cube.rot_x += 1.0 * dt as f32;
    state.cube.rot_y += 2.0 * dt as f32;

    sgl::defaults();
    sgl::load_pipeline(state.pip3d);

    sgl::matrix_mode_projection();
    sgl::perspective(sgl::rad(45.0), 1.0, 0.1, 100.0);

    sgl::matrix_mode_modelview();
    sgl::translate(0.0, 0.0, -12.0);
    sgl::rotate(sgl::rad(state.cube.rot_x), 1.0, 0.0, 0.0);
    sgl::rotate(sgl::rad(state.cube.rot_y), 0.0, 1.0, 0.0);
    draw_cube();
    sgl::push_matrix();
    sgl::translate(0.0, 0.0, 3.0);
    sgl::scale(0.5, 0.5, 0.5);
    sgl::rotate(-2.0 * sgl::rad(state.cube.rot_x), 1.0, 0.0, 0.0);
    sgl::rotate(-2.0 * sgl::rad(state.cube.rot_y), 0.0, 1.0, 0.0);
    draw_cube();
    sgl::push_matrix();
    sgl::translate(0.0, 0.0, 3.0);
    sgl::scale(0.5, 0.5, 0.5);
    sgl::rotate(-3.0 * sgl::rad(state.cube.rot_x), 1.0, 0.0, 0.0);
    sgl::rotate(3.0 * sgl::rad(state.cube.rot_y), 0.0, 0.0, 1.0);
    draw_cube();
    sgl::pop_matrix();
    sgl::pop_matrix();
}

fn draw_tex_cube(state: &mut State, dt: f64) {
    state.texcube.time_accum += dt as f32;
    let a = sgl::rad(state.texcube.time_accum);

    // texture matrix rotation and scale
    let tex_rot = a * 0.5;
    let tex_scale = 1.0 + a.sin() * 0.5;

    // compute an orbiting eye position for testing sgl::lookat()
    let eye_x = a.sin() * 6.0;
    let eye_y = a.sin() * 3.0;
    let eye_z = a.cos() * 6.0;

    sgl::defaults();
    sgl::load_pipeline(state.pip3d);

    sgl::enable_texture();
    sgl::texture(state.tex_view, state.smp);

    sgl::matrix_mode_projection();
    sgl::perspective(sgl::rad(45.0), 1.0, 0.1, 100.0);
    sgl::matrix_mode_modelview();
    sgl::lookat(eye_x, eye_y, eye_z, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    sgl::matrix_mode_texture();
    sgl::rotate(tex_rot, 0.0, 0.0, 1.0);
    sgl::scale(tex_scale, tex_scale, 1.0);
    draw_cube();
}

fn main() {
    let state = Box::new(State::default());

    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        user_data,
        window_title: c"sql.rs".as_ptr(),
        width: 512,
        height: 512,
        sample_count: 4,
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        ..Default::default()
    });
}
