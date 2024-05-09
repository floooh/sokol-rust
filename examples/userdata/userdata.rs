use sokol::{app as sapp, gfx as sg, glue as sglue};

#[derive(Debug)]
pub struct ExampleUserData {
    data: u8,

    map: std::collections::HashMap<u8, u8>,
}

extern "C" fn init() {
    sg::setup(&sg::Desc {
        environment: sokol::glue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });
}

extern "C" fn frame_userdata(userdata: *mut core::ffi::c_void) {
    /*
        3. We then need to turn the raw c pointer into a mutable reference to the same
           type as we created in main. This is safe as long as the data was indeed provided
           and the type is the same.
    */
    let state: &mut ExampleUserData = unsafe { &mut *(userdata as *mut _) };

    /*
        4. Just randomly modifying the data here for demonstration
    */
    state.data = state.data.wrapping_add(1);
    if state.data % 13 == 0 {
        let val = (state.data.wrapping_mul(13)) / 3;
        state.map.insert(state.data, val);
    }
    if state.data % 12 == 0 && state.data % 15 == 0 {
        state.map.clear();
    }
    println!("{state:?}");

    sg::begin_pass(&sg::Pass { swapchain: sglue::swapchain(), ..Default::default() });
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    sg::shutdown()
}

fn main() {
    let mut user_data = ExampleUserData { data: 0, map: std::collections::HashMap::default() };

    sapp::run(&sapp::Desc {
        /*
            1. 'user_data' is allocated on the stack in the main function and we take a
               mutable reference to it which we cast to a pointer and pass to sokol
        */
        user_data: &mut user_data as *mut _ as _,
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        /*
            2. We can use the userdata callbacks to get the userdata passed as an argument,
               but we could also use the normal callbacks and call sapp::userdata() to
               fetch the data manually
        */
        frame_userdata_cb: Some(frame_userdata),
        width: 800,
        height: 600,
        sample_count: 4,
        window_title: c"userdata.rs".as_ptr(),
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        ..Default::default()
    });
}
