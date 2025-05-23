//------------------------------------------------------------------------------
//  debugtext/main.rs
//
//  Text rendering with sokol_debugtext.h, test builtin fonts.
//------------------------------------------------------------------------------

use std::ffi;

use sokol::{app as sapp, debugtext as sdtx, gfx as sg, glue as sglue};

const FONT_KC853: usize = 0;
const FONT_KC854: usize = 1;
const FONT_Z1013: usize = 2;
const FONT_CPC: usize = 3;
const FONT_C64: usize = 4;
const FONT_ORIC: usize = 5;

struct State {
    pass_action: sg::PassAction,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };
    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.125, b: 0.25, a: 1.0 },
        ..Default::default()
    };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    let mut desc = sdtx::Desc::new();
    desc.fonts[FONT_KC853] = sdtx::font_kc853();
    desc.fonts[FONT_KC854] = sdtx::font_kc854();
    desc.fonts[FONT_Z1013] = sdtx::font_z1013();
    desc.fonts[FONT_CPC] = sdtx::font_cpc();
    desc.fonts[FONT_C64] = sdtx::font_c64();
    desc.fonts[FONT_ORIC] = sdtx::font_oric();
    sdtx::setup(&desc)
}

fn print_font(font_index: usize, title: &str, r: u8, g: u8, b: u8) {
    sdtx::font(font_index);
    sdtx::color3b(r, g, b);
    sdtx::puts(title);

    for c in 32_i32..=255 {
        sdtx::putc(c as _);
        if ((c + 1) & 63) == 0 {
            sdtx::crlf();
        }
    }

    sdtx::crlf();
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };
    // set virtual canvas size to half display size so that
    // glyphs are 16x16 display pixels
    sdtx::canvas(sapp::widthf() * 0.5, sapp::heightf() * 0.5);
    sdtx::origin(0.0, 2.0);
    sdtx::home();

    print_font(FONT_KC853, "KC85/3:\n", 0xf4, 0x43, 0x36);
    print_font(FONT_KC854, "KC85/4:\n", 0x21, 0x96, 0xf3);
    print_font(FONT_Z1013, "Z1013:\n", 0x4c, 0xaf, 0x50);
    print_font(FONT_CPC, "Amstrad CPC:\n", 0xff, 0xeb, 0x3b);
    print_font(FONT_C64, "C64:\n", 0x79, 0x86, 0xcb);
    print_font(FONT_ORIC, "Oric Atmos:\n", 0xff, 0x98, 0x00);

    sg::begin_pass(&sg::Pass {
        action: state.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sdtx::draw();
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sdtx::shutdown();
    sg::shutdown();

    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    let state = Box::new(State { pass_action: sg::PassAction::new() });

    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        user_data,
        width: 1024,
        height: 600,
        window_title: c"debugtext.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        ..Default::default()
    })
}
