use glutin::event_loop::ControlFlow;
use glutin::event::KeyboardInput;
use glutin::window::Window;
use cgmath::{Vector3, Point3};


mod renderer;

mod view;
mod controls;

mod item;

use crate::settings::*;
use crate::game::GameMode::Menu;
use std::f32::consts::PI;

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
        z: 5.0,
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


impl Game {
    pub fn load_scene(&mut self) {
        let r = self.item_handler.add_new_model("./src/game/item/loader/res/ball.dae",
                                                "./src/game/item/loader/res/Untitled.001.png");
        r.translate(Vector3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        });

        r.set_rotation_speed_y(PI / 4.0);

        let r = self.item_handler.add_new_model("./src/game/item/loader/res/ball.dae",
                                                "./src/game/item/loader/res/Untitled.001.png");
        r.translate(Vector3 {
            x: -2.0,
            y: 0.0,
            z: 0.0,
        });

        r.set_rotation_speed_y(-PI / 4.0);

        self.item_handler.add_light_source(item::lighting::new_light_source(
            Vector3{
                x: 5.0,
                y: 0.0,
                z: 5.0
            },
            Vector3{
                x: 1.0,
                y: 0.0,
                z: 0.0
            },
            20.0
        ));
        self.item_handler.add_light_source(item::lighting::new_light_source(
            Vector3{
                x: -5.0,
                y: 0.0,
                z: 5.0
            },
            Vector3{
                x: 0.0,
                y: 1.0,
                z: 0.0
            },
            20.0
        ));
        self.item_handler.add_light_source(item::lighting::new_light_source(
            Vector3{
                x: 5.0,
                y: 0.0,
                z: -5.0
            },
            Vector3{
                x: 0.0,
                y: 0.0,
                z: 1.0
            },
            20.0
        ));
        self.item_handler.add_light_source(item::lighting::new_light_source(
            Vector3{
                x: -5.0,
                y: 0.0,
                z: -5.0
            },
            Vector3{
                x: 1.0,
                y: 1.0,
                z: 1.0
            },
            20.0
        ));

    }
    pub fn handle_key_inputs(&mut self, input: &KeyboardInput) -> ControlFlow {
        match input.virtual_keycode {
            Some(key) => {
                match self.game_mode {
                    GameMode::Playing => {
                        use glutin::event::VirtualKeyCode::*;
                        use glutin::event::ElementState::*;
                        match key {
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
                            LShift => if input.state == Pressed { self.camera.set_speed(5.0) } else { self.camera.set_speed(1.0) },
                            _ => ()
                        }
                    }
                    GameMode::Menu => {
                        use glutin::event::VirtualKeyCode::*;
                        use glutin::event::ElementState::*;
                        match key {
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
            }
            None => ()
        }

        ControlFlow::Poll
    }

    pub fn handle_cursor_movement(&mut self, delta_x: f32, delta_y: f32) {
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


        let (pos,col,pow) = self.item_handler.get_nearest_light_data();
        self.renderer.set_uniform_light_positions_worldspace(pos);
        self.renderer.set_uniform_light_colors(col);
        self.renderer.set_uniform_light_powers(pow);

        self.renderer.set_uniform_v(view_matrix);

        for m in &mut self.item_handler.model_sets {
            self.renderer.set_texture(m.1.get_texture());

            m.1.enable_buffers();

            for t in &mut m.0 {
                let model_matrix = t.get_intr_model_matrix(interpolation_value);

                self.renderer.set_uniform_m(model_matrix);
                self.renderer.set_uniform_mvp(model_matrix, view_matrix);

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
