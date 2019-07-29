use crate::settings;
use glutin::event_loop::EventLoop;
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};



pub fn initialize_window(vsync: bool) -> (EventLoop<()>, ContextWrapper<PossiblyCurrent, Window>) {
    let event_loop = glutin::event_loop::EventLoop::new_user_event();
    let wb = glutin::window::WindowBuilder::new().with_title(settings::WINDOW_LABEL);
    let context = glutin::ContextBuilder::new()
        .with_vsync(vsync)
        .build_windowed(wb, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let windowed_context = unsafe { context.make_current().unwrap() };

    // Load the OpenGL function pointers
    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        //gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );
    }
    (event_loop, windowed_context)
}

use std::time::{Duration, Instant};

static ONE_SEC: Duration = Duration::from_secs(1);

pub struct FpsUpsCounter {
    current_time: Instant,
    fps_counter: u32,
    ups_counter: u32,
}

pub fn new_fps_ups_counter() -> FpsUpsCounter {
    FpsUpsCounter {
        current_time: Instant::now(),
        fps_counter: 0,
        ups_counter: 0,
    }
}


impl FpsUpsCounter {
    pub fn advance_fps(&mut self) {
        self.fps_counter += 1;
    }
    pub fn advance_ups(&mut self) {
        self.ups_counter += 1;
    }
    pub fn display_if_one_sec_over(&mut self) {
        if self.current_time.elapsed() >= ONE_SEC {
            println!("frames/sec: {}", self.fps_counter);
            println!("updates/sec: {}\n", self.ups_counter);
            self.current_time = Instant::now();
            self.fps_counter = 0;
            self.ups_counter = 0;
        }
    }
}