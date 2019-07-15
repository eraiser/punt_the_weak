extern crate gl;
extern crate glutin;

use std::path::MAIN_SEPARATOR;

mod window_utilities;

mod game;

/*
SETTINGS
*/
static VSYNC: bool = true;

fn main() {
    let (event_loop, windowed_context)
        = window_utilities::initialize_window(VSYNC);

    let mut game = game::new_game();
    game.load_scene();


    use std::time::{Duration, Instant};

    let mut now = Instant::now();
    let one_sec = Duration::from_secs(1);
    let mut frame_c = 0;
    let mut update_c = 0;

    let game_time = Instant::now();
    let mut previous = game_time.elapsed();
    let mut lag = Duration::from_micros(0);
    use game::MCS_PER_UPDATE;


    use glutin::event_loop::ControlFlow;
    use glutin::event::{WindowEvent, Event};

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor =
                        windowed_context.window().hidpi_factor();
                    windowed_context
                        .resize(logical_size.to_physical(dpi_factor));

                    unsafe {
                        gl::Viewport(0, 0, logical_size.width as i32, logical_size.height as i32);
                    }
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    game.cleanup();
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    *control_flow = game.handle_key_inputs(input);
                }
                _ => (),
            },

            Event::EventsCleared => {
                if control_flow != &ControlFlow::Exit {


                    let current = game_time.elapsed();
                    let elapsed = current - previous;
                    previous = current;
                    lag += elapsed;

                    while lag >= MCS_PER_UPDATE {
                        game.update();
                        lag -= MCS_PER_UPDATE;
                        update_c+=1;
                    }

                    frame_c+=1;
                    if now.elapsed() >= one_sec {
                        println!("frames/sec: {}",frame_c);
                        println!("updates/sec: {}\n",update_c);
                        now = Instant::now();
                        frame_c = 0;
                        update_c = 0;
                    }

                    game.draw((lag.as_micros() as f32) / (MCS_PER_UPDATE.as_micros() as f32));
                    windowed_context.swap_buffers().unwrap();
                }
            }
            _ => (),
        }
    });
}