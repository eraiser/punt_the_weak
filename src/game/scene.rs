use crate::game::{Game, item};
use cgmath::{Vector3, InnerSpace, Vector2};
use std::f32::consts::PI;

pub fn load_scene(g:&mut Game){

    use rand::Rng;

    let speed: f32 = 5.0;
    let mut rng = rand::thread_rng();

    for _x in 0..1 {
        let r = g.item_handler.add_new_model("./res/ball.dae",
                                                "./res/Untitled.001.png");
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

        //r.set_movement_vector(v);

        r.set_rotation_speed_y(PI / 4.0);
    }

    g.item_handler
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
    g.item_handler
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
    g.item_handler
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
    g.item_handler
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
    let t = g.item_handler.add_new_sprite_string("Hello World\nHello World");
    t.set_offset(Vector2{ x: 100.0, y: 200.0 });
    t.set_dimensions(Vector2{ x: 300.0, y: 200.0 });
}