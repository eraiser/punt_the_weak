extern crate collada;
extern crate gl;
extern crate glutin;
extern crate image;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate lazy_static;

use glutin::dpi::LogicalPosition;
use glutin::dpi::PhysicalSize;

use settings::*;

mod game;
mod window_utilities;

mod settings;

mod common_consts;

mod esp_mpu_handler;


fn main() {
    esp_mpu_handler::start_ct();

    let (event_loop, windowed_context) = window_utilities::initialize_window(VSYNC);

    let mut game = game::new_game();
    game.load_scene();

    let mut window_size = windowed_context.window().inner_size();

    use std::time::{Duration, Instant};

    let mut fps_ups_counter = window_utilities::new_fps_ups_counter();

    let game_time = Instant::now();
    let mut previous = game_time.elapsed();
    let mut lag = Duration::from_micros(0);

    use glutin::event::{DeviceEvent, Event, WindowEvent};
    use glutin::event_loop::ControlFlow;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::Resized(logical_size) => {
                window_size = *logical_size;
                game.handle_screen_resolution_change(window_size.width as f32,window_size.height as f32);
                unsafe {
                    gl::Viewport(0, 0, logical_size.width as i32, logical_size.height as i32);
                }
            }
            WindowEvent::KeyboardInput { input, .. } => {
                *control_flow = game.handle_key_inputs(input);
            }
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
                game.cleanup();
                esp_mpu_handler::shut_down();
            }
            _ => (),
        },

        Event::DeviceEvent { ref event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
                let x_movement = delta.0  as f32 / window_size.width as f32;
                let y_movement = delta.1  as f32/ window_size.height as f32;
                game.handle_cursor_movement(x_movement, y_movement);
            }
            _ => (),
        },

        Event::MainEventsCleared => {
            if *control_flow != ControlFlow::Exit {
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
    });
}
