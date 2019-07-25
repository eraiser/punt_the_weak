use glutin::event_loop::ControlFlow;
use glutin::event::KeyboardInput;
use glutin::window::Window;
use cgmath::{Vector3, Point3};


mod renderer;
mod model;

mod view;
mod controls;
mod loader;

use crate::settings::*;
use crate::game::GameMode::Menu;

enum GameMode {
    Menu,
    Playing,
}

pub struct Game {
    renderer: renderer::Renderer,
    items: Vec<model::Model>,
    camera: view::Camera,
    controls: controls::Controls,
    game_mode: GameMode,
    game_mode_changed: bool,
}


pub fn new_game() -> Game {
    let cam_pos = Point3 {
        x: 0.0,
        y: 0.0,
        z: 5.0,
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
        game_mode: GameMode::Playing,
        game_mode_changed: true,
    }
}


impl Game {
    pub fn load_scene(&mut self) {



        self.items.push(model::triangle());
        //self.items[0].set_rotation_speed_x(std::f32::consts::PI);
        //self.items[0].set_rotation_speed_y(std::f32::consts::PI);
        //self.items[0].set_rotation_speed_z(std::f32::consts::PI);
        //self.items[0].set_movement_vector(Vector3{x:1.0,y:0.0,z:0.0});
    }

    pub fn handle_key_inputs(&mut self, input: &KeyboardInput) -> ControlFlow {
        match self.game_mode {
            GameMode::Playing => {
                use glutin::event::VirtualKeyCode::*;
                use glutin::event::ElementState::*;
                match input.virtual_keycode.unwrap() {
                    W => if input.state == Pressed { self.controls.forward = true } else { self.controls.forward = false },
                    S => if input.state == Pressed { self.controls.backward = true } else { self.controls.backward = false },
                    A => if input.state == Pressed { self.controls.left = true } else { self.controls.left = false },
                    D => if input.state == Pressed { self.controls.right = true } else { self.controls.right = false },
                    X => if input.state == Pressed { self.controls.down = true } else { self.controls.down = false },
                    Y => if input.state == Pressed { self.controls.up = true } else { self.controls.up = false },
                    F => if input.state == Pressed {
                        match self.game_mode {
                            GameMode::Playing => self.game_mode = Menu,
                            GameMode::Menu => self.game_mode = GameMode::Playing
                        }
                        self.game_mode_changed = true;
                    }
                    Escape => {
                        self.cleanup();
                        return ControlFlow::Exit;
                    }
                    _ => ()
                }
            }
            GameMode::Menu => {
                use glutin::event::VirtualKeyCode::*;
                use glutin::event::ElementState::*;
                match input.virtual_keycode.unwrap() {
                    F => if input.state == Pressed {
                        match self.game_mode {
                            GameMode::Playing => self.game_mode = Menu,
                            GameMode::Menu => self.game_mode = GameMode::Playing
                        }
                        self.game_mode_changed = true;
                    }
                    Escape => {
                        self.cleanup();
                        return ControlFlow::Exit;
                    }
                    _ => ()
                }
            }
        }
        ControlFlow::Poll
    }

    pub fn handle_cursor_movement(&mut self, delta_x: f32, delta_y: f32) {
        use std::f32::consts::PI;
        match self.game_mode {
            GameMode::Playing => {
                self.camera.rotate_x(PI * -delta_y);
                self.camera.rotate_y(PI * -delta_x);
            }
            GameMode::Menu => ()
        }
    }

    pub fn update(&mut self, window: &Window) {
        let mv = self.controls.get_movement_vec(self.camera.get_current_dir()
                                                , self.camera.get_speed() / TICKS_PER_SECOND as f32);
        self.camera.move_dir(mv);

        self.items[0].update();


        if self.game_mode_changed {
            match self.game_mode {
                GameMode::Playing => {
                    window.set_cursor_grab(true).unwrap();
                    window.set_cursor_visible(false);
                    self.controls.reset();
                }
                GameMode::Menu => {
                    window.set_cursor_grab(false).unwrap();
                    window.set_cursor_visible(true);
                    self.controls.reset();
                }
            }
            self.game_mode_changed = false;
        }
    }

    pub fn draw(&mut self, interpolation_value: f32) {
        unsafe {
            gl::ClearColor(0.0, 1.0, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.renderer.use_3d_program();
        let vm = self.camera.get_int_view_matrix(&interpolation_value);
        for m in self.items.iter() {
            self.renderer.set_mvp(m.get_int_model_matrix(&interpolation_value), vm);
            self.renderer.set_texture(m.get_texture());
            m.draw_3d();
        }
    }


    pub fn cleanup(&self) {
        self.renderer.cleanup();

        for i in self.items.iter() {
            i.cleanup();
        }
    }
}
