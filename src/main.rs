extern crate gl;
extern crate glutin;
extern crate image;
extern crate collada;

mod window_utilities;
mod game;

mod settings;

use settings::*;

fn main() {
    let (event_loop, windowed_context)
        = window_utilities::initialize_window(VSYNC);

    let mut game = game::new_game();
    game.load_scene();

    let mut window_size = windowed_context.window().inner_size();

    use std::time::{Duration, Instant};

    let mut fps_ups_counter = window_utilities::new_fps_ups_counter();

    let game_time = Instant::now();
    let mut previous = game_time.elapsed();
    let mut lag = Duration::from_micros(0);

    use glutin::event_loop::ControlFlow;
    use glutin::event::{DeviceEvent, WindowEvent, Event};


    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor =
                        windowed_context.window().hidpi_factor();
                    windowed_context
                        .resize(logical_size.to_physical(dpi_factor));
                    window_size = *logical_size;
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


            Event::DeviceEvent {ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta} => {
                    let x_movement = delta.0/window_size.width;
                    let y_movement = delta.1/window_size.height;
                    game.handle_cursor_movement(x_movement as f32,y_movement as f32);
                }
                _ => (),
            }

            Event::EventsCleared => {
                if control_flow != &ControlFlow::Exit {

                    let current = game_time.elapsed();
                    let elapsed = current - previous;
                    previous = current;
                    lag += elapsed;

                    while lag >= MCS_PER_UPDATE {
                        game.update(windowed_context.window());
                        lag -= MCS_PER_UPDATE;
                        fps_ups_counter.advance_ups();
                    }

                    fps_ups_counter.advance_fps();
                    fps_ups_counter.display_if_one_sec_over();

                    game.draw((lag.as_micros() as f32) / (MCS_PER_UPDATE.as_micros() as f32));
                    windowed_context.swap_buffers().unwrap();
                }
            }
            _ => (),
        }
    });
}