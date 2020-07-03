use crate::game::{Game, item};
use cgmath::{Vector3, InnerSpace, Vector2, Rad, Quaternion,Rotation3};
use std::f32::consts::PI;
use item::model::motion;

pub fn load_scene(g:&mut Game){

    let mut plain = g.item_handler.add_new_model(   "./res/plain.dae",
                                                                "./res/Untitled.002.png");
    plain.transform.rotate_x(Rad(PI/2.0));
    plain.transform.translate(Vector3{
        x: -5.0,
        y: -2.0,
        z: -5.0
    });
    plain.transform.scale(5.0);
    plain.calc_model_matrix(0.0);


    for _x in 0..1 {
        let r = g.item_handler.add_new_model(   "./res/ball.dae",
                                                            "./res/Untitled.001.png");
        r.transform.rotate_y(Rad(PI/2.0));
        let mut ts = motion::new_motion();
        ts.rotation_per_sec = Rotation3::from_angle_y(Rad(std::f32::consts::PI/100.0));
        r.motion = Some(ts);
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
    t.set_offset(Vector2{ x: 0.0, y: 0.0 });
    t.set_dimensions(Vector2{ x: 300.0, y: 200.0 });
}