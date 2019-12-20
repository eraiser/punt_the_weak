use std::f32::consts::PI;

use cgmath::{InnerSpace, Point3, Vector3};
use glutin::event::KeyboardInput;
use glutin::event_loop::ControlFlow;
use glutin::window::Window;

use crate::game::GameMode::Menu;
use crate::settings::*;

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
    text_2d: item::mesh::sprite2d::Sprite2D
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
        text_2d: item::mesh::new_2d_text("Hello World!\nU gay")
    }
}

impl Game {
    pub fn load_scene(&mut self) {
        use rand::Rng;

        let speed: f32 = 5.0;
        let mut rng = rand::thread_rng();

        for _x in 0..10 {
            let r = self.item_handler.add_new_model("C:/Users/krott/Documents/RustProjekt/punt_the_weak/src/game/item/loader/res/ball.dae",
                                                    "C:/Users/krott/Documents/RustProjekt/punt_the_weak/src/game/item/loader/res/Untitled.001.png");
            r.translate(Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            });

            let mut v: Vector3<f32> = Vector3 {
                x: (rng.gen::<f32>() * speed) - (speed / 2.0),
                y: (rng.gen::<f32>() * speed) - (speed / 2.0),
                z: (rng.gen::<f32>() * speed) - (speed / 2.0),
            };

            v = v.normalize();

            r.set_movement_vector(v);

            r.set_rotation_speed_y(PI / 4.0);
        }

        self.item_handler
            .add_light_source(item::lighting::new_light_source(
                Vector3 {
                    x: 5.0,
                    y: 0.0,
                    z: 5.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                20.0,
            ));
        self.item_handler
            .add_light_source(item::lighting::new_light_source(
                Vector3 {
                    x: -5.0,
                    y: 0.0,
                    z: 5.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                20.0,
            ));
        self.item_handler
            .add_light_source(item::lighting::new_light_source(
                Vector3 {
                    x: 5.0,
                    y: 0.0,
                    z: -5.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                20.0,
            ));
        self.item_handler
            .add_light_source(item::lighting::new_light_source(
                Vector3 {
                    x: -5.0,
                    y: 0.0,
                    z: -5.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                20.0,
            ));
    }
    pub fn handle_key_inputs(&mut self, input: &KeyboardInput) -> ControlFlow {
        use glutin::event::ElementState::*;
        use glutin::event::VirtualKeyCode::*;

        match input.virtual_keycode {
            Some(key) => match self.game_mode {
                GameMode::Playing => match key {
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

    pub fn update(&mut self, window: &Window) {
        let mv = self.controls.get_movement_vec(
            self.camera.get_current_dir(),
            self.camera.get_speed() / TICKS_PER_SECOND as f32,
        );
        self.camera.move_dir(mv);

        self.item_handler.update();

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
        self.renderer.set_texture(self.text_2d.texture);
        self.text_2d.enable_buffers();
        self.text_2d.draw();
        self.text_2d.disable_buffers();
    }

    pub fn cleanup(&self) {
        self.renderer.cleanup();
        self.item_handler.cleanup();
    }
}
