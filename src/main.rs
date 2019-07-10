extern crate gl;
extern crate glutin;

use gl::types::*;


use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;


use glutin::event::{ WindowEvent, Event};

mod window_utilities;

mod game;

fn main() {
    let mut w = window_utilities::initialize_window();
    let mut ge = game::new_game_engine(w.0,w.1);
    let mut g = game::punt_the_weak::new_game();

    ge.start_game(g);
}