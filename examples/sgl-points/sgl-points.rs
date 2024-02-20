//------------------------------------------------------------------------------
//  sgl-points/sgl-points.rs
//
//  Test point rendering with sokol/gl
//------------------------------------------------------------------------------

use sokol::{app as sapp, gfx as sg, gl as sgl, glue as sglue, log as slog };

#[rustfmt::skip]
const PALETTE: [[f32; 3]; 16] = [
    [ 0.957, 0.263, 0.212 ],
    [ 0.914, 0.118, 0.388 ],
    [ 0.612, 0.153, 0.690 ],
    [ 0.404, 0.227, 0.718 ],
    [ 0.247, 0.318, 0.710 ],
    [ 0.129, 0.588, 0.953 ],
    [ 0.012, 0.663, 0.957 ],
    [ 0.000, 0.737, 0.831 ],
    [ 0.000, 0.588, 0.533 ],
    [ 0.298, 0.686, 0.314 ],
    [ 0.545, 0.765, 0.290 ],
    [ 0.804, 0.863, 0.224 ],
    [ 1.000, 0.922, 0.231 ],
    [ 1.000, 0.757, 0.027 ],
    [ 1.000, 0.596, 0.000 ],
    [ 1.000, 0.341, 0.133 ],
];

extern "C" fn init() {
    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });

    sgl::setup(&sgl::Desc {
        logger: sgl::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
}

fn compute_color(t: f32) -> [f32; 3] {
    // t is expected to be 0.0 <= t <= 1.0
    let i0 = (t * 16.0) as usize % 16;
    let i1 = (i0 + 1) % 16;

    let l = (t * 16.0).fract();
    let c0 = PALETTE[i0];
    let c1 = PALETTE[i1];

    macro_rules! lerp {
        ($a:expr, $b: expr, $t: expr) => {{
            let t = $t;
            (1.0 - t) * $a + t * $b
        }};
    }

    [lerp!(c0[0], c1[0], l), lerp!(c0[1], c1[1], l), lerp!(c0[2], c1[2], l)]
}

extern "C" fn frame() {
    let frame_count = sapp::frame_count();
    let angle = frame_count % 360;

    sgl::defaults();
    sgl::begin_points();
    let mut psize: f32 = 5.0;
    for i in 0..360 {
        let a = ((angle + i) as f32).to_radians();
        let color = compute_color(((frame_count + i) % 300) as f32 / 300.0);
        let r = f32::sin(a * 4.0);
        let s = f32::sin(a);
        let c = f32::cos(a);
        let x = s * r;
        let y = c * r;
        sgl::c3f(color[0], color[1], color[2]);
        sgl::point_size(psize);
        sgl::v2f(x, y);
        psize *= 1.005;
    }
    sgl::end();

    let mut pass_action = sg::PassAction::new();
    pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };
    sg::begin_pass(&sg::Pass {
        action: pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sgl::draw();
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    sgl::shutdown();
    sg::shutdown();
}

fn main() {
    let window_title = b"sgl-points\0".as_ptr() as *const _;

    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        frame_cb: Some(frame),
        cleanup_cb: Some(cleanup),
        width: 512,
        height: 512,
        sample_count: 4,
        window_title,
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        ..Default::default()
    })
}
