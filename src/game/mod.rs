use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::event_loop::EventLoop;
use glutin::window::Window;

mod renderer;
mod model;
pub mod punt_the_weak;

use punt_the_weak::Game;

pub struct GameEngine {
    event_loop: EventLoop<()>,
    windowed_context: ContextWrapper<PossiblyCurrent, Window>,
}

impl GameEngine {
    pub fn start_game(mut self, mut game: Game) {
        use glutin::event::{WindowEvent, Event};

        game.load_scene();
        let event_loop = self.event_loop;
        let mut windowed_context = self.windowed_context;


        event_loop.run(move |event, _, control_flow| {

            game.update();

            unsafe {
                gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                game.draw();

                windowed_context.swap_buffers().unwrap();
            }

            *control_flow = game.handle_events(&mut windowed_context,event);

        });
    }
}

pub fn new_game_engine(event_loop: EventLoop<()>,
                       windowed_context: ContextWrapper<PossiblyCurrent, Window>) -> GameEngine {
    GameEngine {
        event_loop,
        windowed_context,
    }
}
