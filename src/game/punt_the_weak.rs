use crate::game::renderer;
use crate::game::model;
use glutin::event_loop::{ControlFlow};
use glutin::event::{WindowEvent, Event};
use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::window::Window;


pub struct Game {
    renderer: renderer::Renderer,
    items: Vec<model::Model>,
}

pub fn new_game() -> Game {
    Game {
        renderer: renderer::init_renderer(),
        items: Vec::new(),
    }
}

impl Game {
    pub fn handle_events(&mut self
                         , windowed_context: &mut ContextWrapper<PossiblyCurrent, Window>
                         , event: Event<()>) -> ControlFlow {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    windowed_context.resize(logical_size.to_physical(dpi_factor));
                }

                WindowEvent::CloseRequested => {
                    self.cleanup();
                    return ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode.unwrap() {
                    glutin::event::VirtualKeyCode::Escape => {
                        return ControlFlow::Exit;
                    }
                    _ => println!("{:?} {:?}", input.virtual_keycode.unwrap(), input.state)
                }
                _ => (),
            }
            _ => (),
        }
        ControlFlow::Poll
    }
    pub fn draw(&self) {
        self.renderer.use_2d_program();
        for m in self.items.iter() {
            m.draw_2d();
        }
    }
    pub fn update(&mut self) {}

    fn cleanup(&self) {
        self.renderer.cleanup();
        for m in self.items.iter() {
            m.cleanup();
        }
    }

    pub fn load_scene(&mut self) {
        self.items.push(model::triangle())
    }
}
