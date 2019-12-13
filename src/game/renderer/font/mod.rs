pub struct FontData {
    texture_font: u32,
    image_width: u16,
    image_height: u16,
    cell_width: u16,
    cell_height: u16,

    start_char: u16,
    font_height: u16,

    widths: Vec<u16>,
}

use gl::types::*;
use std::fmt::{Formatter, Error};

impl FontData {
    pub fn load_font() -> FontData{

        let mut fd = FontData{
            texture_font: crate::game::item::loader::load_texture("C:/Users/krott/Documents/RustProjekt/punt_the_weak/src/game/renderer/font/fonts/ExportedFont.bmp"),
            image_width: 0,
            image_height: 0,
            cell_width: 0,
            cell_height: 0,
            start_char: 0,
            font_height: 0,
            widths: Vec::new()
        };

        let csv_file = include_str!("fonts/FontData.csv");

        for line in csv_file.lines() {
            let mut words = line.split(' ');
            let w1: &str = words.next().unwrap();

            match w1 {
                "Image" => {
                    let mut temp = words.next().unwrap().split(',');

                    let w2 = temp.next().unwrap();
                    let v = (temp.next().unwrap()).parse::<u16>().unwrap();
                    match w2 {
                        "Width" => fd.image_width = v,
                        "Height" => fd.image_height = v,
                        _ => ()
                    }
                },
                "Cell" => {
                    let mut temp = words.next().unwrap().split(',');

                    let w2 = temp.next().unwrap();
                    let v = (temp.next().unwrap()).parse::<u16>().unwrap();
                    match w2 {
                        "Width" => fd.cell_width = v,
                        "Height" => fd.cell_height = v,
                        _ => ()
                    }
                },
                "Start" => {
                    let mut temp = words.next().unwrap().split(',');

                    temp.next();
                    let v = (temp.next().unwrap()).parse::<u16>().unwrap();
                    fd.start_char = v;
                },
                "Font" => {
                    let mut temp = words.next().unwrap().split(',');

                    let w2 = temp.next().unwrap();
                    match w2 {
                        "Height" => {
                            let v = (temp.next().unwrap()).parse::<u16>().unwrap();
                            fd.font_height = v;
                        },
                        _ => ()
                    }
                },
                "Char" => {
                    words.next();
                    let w2 :&str = words.next().unwrap();
                    if !(w2.starts_with("Base")) {
                        return fd;
                    } else {
                        let mut temp = words.next().unwrap().split(',');
                        temp.next();
                        let v = (temp.next().unwrap()).parse::<u16>().unwrap();
                        fd.widths.push(v);
                    }
                }
                _ => ()
            }
        }
        fd
    }
}

use std::fmt;
impl fmt::Display for FontData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "image_width: {}\n",self.image_width);
        write!(f, "image_height: {}\n",self.image_height);
        write!(f, "cell_width: {}\n",self.cell_width);
        write!(f, "cell_height: {}\n",self.cell_height);
        write!(f, "start_char: {}\n",self.start_char);
        write!(f, "font_height: {}\n",self.font_height);
        write!(f, "font_widths: {:?}",self.widths)
    }
}
