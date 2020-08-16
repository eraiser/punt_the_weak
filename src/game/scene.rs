use crate::game::{Game, item};
use cgmath::{Vector3, InnerSpace, Vector2, Rad, Quaternion,Rotation3};
use std::f32::consts::PI;
use item::model::motion;
use crate::game::item::model::motion::MotionType::q_to_q;
use crate::game::item::model::motion::MotionType;

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


    let r = g.item_handler.add_new_model(   "./res/ball.dae",
                                                        "./res/Untitled.001.png");
    r.transform.rotate_z(Rad(PI/2.0));
    r.transform.translation = Vector3::new(0.0,0.0,0.0);

    let mut ts = motion::new_q_to_q_motion();
    r.motion = Some(MotionType::q_to_q(ts));

    let r = g.item_handler.add_new_model(   "./res/untitled.dae",
                                                        "./res/Untitled.001.png");
    r.transform.rotate_x(Rad(3.0*PI/2.0));
    r.transform.translation = Vector3::new(5.0,0.0,0.0);


    let mut ts = motion::new_continuous_motion();
    ts.set_rotation_per_sec_y(Rad(std::f32::consts::PI/3.0));
    r.motion = Some(MotionType::continuous(ts));

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