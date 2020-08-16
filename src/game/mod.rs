use std::f32::consts::PI;

use cgmath::{InnerSpace, Point3, Vector3, Vector2};
use glutin::event::KeyboardInput;
use glutin::event_loop::ControlFlow;
use glutin::window::Window;

use crate::game::GameMode::Menu;
use crate::settings::*;
use std::borrow::BorrowMut;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::esp_mpu_handler;
use crate::game::item::model::motion::MotionType;

mod renderer;

mod controls;
mod view;

mod item;

enum GameMode {
    Menu,
    Playing,
}

pub struct Game {
    renderer: renderer::Renderer,
    item_handler: item::ItemHandler,
    camera: view::Camera,
    controls: controls::Controls,
    game_mode: GameMode,
    game_mode_changed: bool,
}

pub fn new_game() -> Game {
    let cam_pos = Point3 {
        x: 0.0,
        y: 0.0,
        z: 10.0,
    };
    let cam_dir = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    Game {
        renderer: renderer::init_renderer(),
        item_handler: item::new_item_handler(),
        camera: view::new_camera(cam_pos, cam_dir),
        controls: controls::new_controls(),
        game_mode: GameMode::Playing,
        game_mode_changed: true,
    }
}
mod scene;

static q_pressed:AtomicBool = AtomicBool::new(false);
impl Game {
    pub fn load_scene(&mut self) {
        scene::load_scene(self);
    }

    pub fn handle_key_inputs(&mut self, input: &KeyboardInput) -> ControlFlow {
        use glutin::event::ElementState::*;
        use glutin::event::VirtualKeyCode::*;

        match input.virtual_keycode {
            Some(key) => match self.game_mode {
                GameMode::Playing => match key {
                    Q => {
                        q_pressed.swap(true,Ordering::Relaxed);
                    }
                    W => {
                        if input.state == Pressed {
                            self.controls.forward = true
                        } else {
                            self.controls.forward = false
                        }
                    }
                    S => {
                        if input.state == Pressed {
                            self.controls.backward = true
                        } else {
                            self.controls.backward = false
                        }
                    }
                    A => {
                        if input.state == Pressed {
                            self.controls.left = true
                        } else {
                            self.controls.left = false
                        }
                    }
                    D => {
                        if input.state == Pressed {
                            self.controls.right = true
                        } else {
                            self.controls.right = false
                        }
                    }
                    X => {
                        if input.state == Pressed {
                            self.controls.down = true
                        } else {
                            self.controls.down = false
                        }
                    }
                    Y => {
                        if input.state == Pressed {
                            self.controls.up = true
                        } else {
                            self.controls.up = false
                        }
                    }
                    F => {
                        if input.state == Pressed {
                            match self.game_mode {
                                GameMode::Playing => self.game_mode = Menu,
                                GameMode::Menu => self.game_mode = GameMode::Playing,
                            }
                            self.game_mode_changed = true;
                        }
                    }
                    Escape => {
                        self.cleanup();
                        return ControlFlow::Exit;
                    }
                    LShift => {
                        if input.state == Pressed {
                            self.camera.set_speed(5.0)
                        } else {
                            self.camera.set_speed(1.0)
                        }
                    }
                    _ => (),
                },
                GameMode::Menu => match key {
                    F => {
                        if input.state == Pressed {
                            match self.game_mode {
                                GameMode::Playing => self.game_mode = Menu,
                                GameMode::Menu => self.game_mode = GameMode::Playing,
                            }
                            self.game_mode_changed = true;
                        }
                    }
                    Escape => {
                        self.cleanup();
                        return ControlFlow::Exit;
                    }
                    _ => (),
                },
            },
            None => (),
        }

        ControlFlow::Poll
    }

    pub fn handle_cursor_movement(&mut self, delta_x: f32, delta_y: f32) {
        match self.game_mode {
            GameMode::Playing => {
                self.camera.rotate_x(PI * -delta_y);
                self.camera.rotate_y(PI * -delta_x);
            }
            GameMode::Menu => (),
        }
    }

    pub fn handle_screen_resolution_change(&mut self, width: f32, heidht: f32)
    {
        self.renderer.handle_screen_resolution_change(width,heidht);
    }

    pub fn update(&mut self, window: &Window) {
        use cgmath::{Rotation3,Rad,Quaternion};
        let mv = self.controls.get_movement_vec(
            self.camera.get_current_dir(),
            self.camera.get_speed() / TICKS_PER_SECOND as f32,
        );
        self.camera.move_dir(mv);

        self.item_handler.update();


        let model = self.item_handler.get_model(0);
        match esp_mpu_handler::get_value(0) {
            Some(f)=> {

                match model.motion.borrow_mut() {
                    Some(m) => {
                        match m {
                            MotionType::q_to_q(q) => {
                                let r = Quaternion::new(f.0,f.1,f.2,f.3);
                                q.target_quaternion = r;
                            },
                            _ => ()
                        }
                    },
                    None => ()
                }

            }
            None => println!("no update")
        }

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

        let view_matrix = self.camera.get_int_view_matrix(interpolation_value);

        let (pos, col, pow) = self.item_handler.get_nearest_light_data();
        self.renderer.set_uniform_light_positions_worldspace(pos);
        self.renderer.set_uniform_light_colors(col);
        self.renderer.set_uniform_light_powers(pow);

        self.renderer.set_uniform_v(view_matrix);

        self.item_handler
            .calc_intp_modelmatrices(interpolation_value);

        for m in &mut self.item_handler.model_sets {
            self.renderer.set_texture(m.1.get_texture());

            m.1.enable_buffers();

            for t in &mut m.0 {
                let model_matrix = t.get_current_model_matrix();

                self.renderer.set_uniform_m(model_matrix);
                self.renderer.set_uniform_mvp(model_matrix, view_matrix);

                m.1.draw();
            }

            m.1.disable_buffers();
        }

        self.renderer.use_2d_program();
        let mut l = 0.0;
        for m in &mut self.item_handler.sprite_sets {
            self.renderer.set_texture(m.1.get_texture());

            m.1.enable_buffers();
            for t in &mut m.0 {
                self.renderer.set_uniform_ortho();
                self.renderer.set_uniform_offset(t.get_offset());
                self.renderer.set_uniform_dimensions(t.get_dimensions());
                self.renderer.set_uniform_level(l);
                l-=1.0;
                m.1.draw();
            }

            m.1.disable_buffers();
        }
    }

    pub fn cleanup(&self) {
        self.renderer.cleanup();
        self.item_handler.cleanup();
    }
}
