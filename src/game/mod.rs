use glutin::event_loop::{EventLoopProxy, EventLoop, ControlFlow};
use glutin::event::{WindowEvent, Event};
use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::window::Window;
use std::sync::mpsc::Receiver;
use glutin::event::{VirtualKeyCode, KeyboardInput};
use cgmath::{Vector3, Point3};
use std::time::Duration;


mod renderer;
mod model;

mod view;
mod controls;


static TICKS_PER_SECOND: u64 = 25;
pub static MCS_PER_UPDATE: Duration = Duration::from_micros(1000000 / TICKS_PER_SECOND);
// 25 TICKS

pub struct Game {
    renderer: renderer::Renderer,
    items: Vec<model::Model>,
    camera: view::Camera,
    controls: controls::Controls,
}

pub fn new_game() -> Game {
    let cam_pos = Point3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    let cam_dir = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    Game {
        renderer: renderer::init_renderer(),
        items: Vec::new(),
        camera: view::new_camera(cam_pos, cam_dir),
        controls: controls::new_controls(),
    }
}


impl Game {
    pub fn draw(&mut self, interpolation_value: f32) {
        unsafe {
            gl::ClearColor(0.0, 1.0, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.renderer.use_3d_program();

        for m in self.items.iter() {
            self.renderer.set_mvp(m.get_model_matrix(), self.camera.get_int_view_matrix(&interpolation_value));
            m.draw_3d();
        }
    }

    pub fn handle_key_inputs(&mut self, input: &KeyboardInput) -> ControlFlow {
        use glutin::event::VirtualKeyCode::*;
        use glutin::event::ElementState::*;
        match input.virtual_keycode.unwrap() {
            W => if input.state == Pressed { self.controls.forward = true }     else { self.controls.forward = false },
            S => if input.state == Pressed { self.controls.backward = true }    else { self.controls.backward = false },
            A => if input.state == Pressed { self.controls.left = true }        else { self.controls.left = false },
            D => if input.state == Pressed { self.controls.right = true }       else { self.controls.right = false },
            X => if input.state == Pressed { self.controls.down = true }        else { self.controls.down = false },
            Y => if input.state == Pressed { self.controls.up = true }          else { self.controls.up = false },
            Escape => {
                self.cleanup();
                return ControlFlow::Exit;
            }
            _ => ()
        }
        ControlFlow::Poll
    }

    pub fn update(&mut self) {

        let mv = self.controls.get_movement_vec(self.camera.get_dir()
                                                                    ,self.camera.get_speed()/TICKS_PER_SECOND as f32);

        self.camera.move_dir(mv);

    }


    pub fn cleanup(&self) {
        self.renderer.cleanup();

        for i in self.items.iter() {
            i.cleanup();
        }
    }

    pub fn load_scene(&mut self) {
        self.items.push(model::triangle())
    }
}
