
use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::event_loop::EventLoop;
use glutin::window::Window;


pub fn initialize_window(vsync: bool) -> (EventLoop<()>,ContextWrapper<PossiblyCurrent,Window>)
{
    let event_loop = glutin::event_loop::EventLoop::new_user_event();
    let wb = glutin::window::WindowBuilder::new().with_title("A fantastic window!");
    let context = glutin::ContextBuilder::new().with_vsync(vsync).build_windowed(wb, &event_loop).unwrap();
    
    // It is essential to make the context current before calling `gl::load_with`.
    let windowed_context = unsafe { context.make_current().unwrap() };

    // Load the OpenGL function pointers
    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    (event_loop,windowed_context)
}