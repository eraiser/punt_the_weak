use std::fmt::{Error, Formatter};
use std::fmt;

use cgmath::{Vector2, Vector3};
use gl::types::*;

pub struct FontData {
    image_width: f32,
    image_height: f32,
    cell_width: f32,
    cell_height: f32,

    rows: u16,
    lines: u16,

    start_char: u16,
    font_height: f32,

    widths: Vec<f32>,
}

impl FontData {
    pub fn load_font() -> FontData {
        let mut fd = FontData {
            image_width: 0.0,
            image_height: 0.0,
            cell_width: 0.0,
            cell_height: 0.0,
            rows: 0,
            lines: 0,
            start_char: 0,
            font_height: 0.0,
            widths: Vec::new(),
        };

        let csv_file = include_str!("fonts/FontData.csv");

        for line in csv_file.lines() {
            let mut words = line.split(' ');
            let w1: &str = words.next().unwrap();

            match w1 {
                "Image" => {
                    let mut temp = words.next().unwrap().split(',');

                    let w2 = temp.next().unwrap();
                    let v = (temp.next().unwrap()).parse::<f32>().unwrap();
                    match w2 {
                        "Width" => fd.image_width = v,
                        "Height" => fd.image_height = v,
                        _ => ()
                    }
                }
                "Cell" => {
                    let mut temp = words.next().unwrap().split(',');

                    let w2 = temp.next().unwrap();
                    let v = (temp.next().unwrap()).parse::<f32>().unwrap();
                    match w2 {
                        "Width" => {
                            fd.cell_width = v;
                            fd.rows = (fd.image_width / fd.cell_width) as u16;
                        },
                        "Height" => {
                            fd.cell_height = v;
                            fd.lines = (fd.image_height / fd.cell_height) as u16;
                        },
                        _ => ()
                    }
                }
                "Start" => {
                    let mut temp = words.next().unwrap().split(',');

                    temp.next();
                    let v = (temp.next().unwrap()).parse::<u16>().unwrap();
                    fd.start_char = v;
                }
                "Font" => {
                    let mut temp = words.next().unwrap().split(',');

                    let w2 = temp.next().unwrap();
                    match w2 {
                        "Height" => {
                            let v = (temp.next().unwrap()).parse::<u16>().unwrap();
                            fd.font_height = v as f32;
                        }
                        _ => ()
                    }
                }
                "Char" => {
                    let v = (words.next().unwrap()).parse::<u16>().unwrap();
                    let w2: &str = words.next().unwrap();
                    if !(w2.starts_with("Base")) {
                        break;
                    } else {
                        if v >= fd.start_char{
                            let mut temp = words.next().unwrap().split(',');
                            temp.next();
                            let v = (temp.next().unwrap()).parse::<u16>().unwrap();
                            fd.widths.push(v as f32);
                        }
                    }
                }
                _ => ()
            }
        }
        fd.lines = (fd.image_height / fd.cell_height) as u16;
        fd.rows = (fd.image_width / fd.cell_width) as u16;
        fd
    }

    pub fn generate_2d_text_vert_uv_data(&self, string: &str) -> (Vec<f32>, Vec<f32>)
    {
        let mut vertices: Vec<f32> = Vec::new();
        let mut uvs: Vec<f32> = Vec::new();

        let mut cursor: Vector2<f32> = Vector2 {
            x: 0.0,
            y: 0.0,
        };
        for c in string.chars() {
            if c.is_ascii() {
                if c == '\n' {
                    cursor.x = 0.0;
                    cursor.y = cursor.y - self.font_height;
                    continue;
                }
                let code: usize = ((c as u16) - self.start_char) as usize;

                let char_width = *self.widths.get(code).unwrap();
                let up_left = cursor;

                let up_right = Vector2 {
                    x: (cursor.x + char_width),
                    y: cursor.y,
                }/self.font_height;
                let down_left = Vector2 {
                    x: cursor.x,
                    y: (cursor.y + self.font_height),
                }/self.font_height;
                let down_right = Vector2 {
                    x: (cursor.x + char_width),
                    y: (cursor.y + self.font_height),
                }/self.font_height;

                vertices.push(up_left.x/self.font_height);
                vertices.push(up_left.y/self.font_height);
                vertices.push(down_left.x);
                vertices.push(down_left.y);
                vertices.push(up_right.x);
                vertices.push(up_right.y);

                vertices.push(down_right.x);
                vertices.push(down_right.y);
                vertices.push(up_right.x);
                vertices.push(up_right.y);
                vertices.push(down_left.x);
                vertices.push(down_left.y);


                cursor.x = cursor.x + (char_width);


                let mut uv_up_left;
                let code = code as f32;
                if code > 0.0 {
                    uv_up_left = Vector2 {
                        x: ((code as u16 % self.rows) as f32 * self.cell_width) / self.image_width,
                        y: 1.0 - (((code as u16 / self.rows) as f32 * self.cell_height) / self.image_height),
                    };
                } else {
                    uv_up_left = Vector2 {
                        x: 0.0,
                        y: 1.0,
                    };
                }

                let uv_up_right = Vector2 {
                    x: uv_up_left.x + (char_width / self.image_width),
                    y: uv_up_left.y,
                };
                let uv_down_left = Vector2 {
                    x: uv_up_left.x,
                    y: uv_up_left.y - (self.cell_height / self.image_height),
                };
                let uv_down_right = Vector2 {
                    x: uv_up_left.x + (char_width / self.image_width),
                    y: uv_up_left.y - (self.cell_height / self.image_height),
                };
                uvs.push(uv_down_left.x);
                uvs.push(uv_down_left.y);
                uvs.push(uv_up_left.x);
                uvs.push(uv_up_left.y);
                uvs.push(uv_down_right.x);
                uvs.push(uv_down_right.y);

                uvs.push(uv_up_right.x);
                uvs.push(uv_up_right.y);
                uvs.push(uv_down_right.x);
                uvs.push(uv_down_right.y);
                uvs.push(uv_up_left.x);
                uvs.push(uv_up_left.y);
            }
        }
        (vertices, uvs)
    }
}

impl fmt::Display for FontData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "image_width: {}\n", self.image_width);
        write!(f, "image_height: {}\n", self.image_height);
        write!(f, "cell_width: {}\n", self.cell_width);
        write!(f, "cell_height: {}\n", self.cell_height);
        write!(f, "start_char: {}\n", self.start_char);
        write!(f, "font_height: {}\n", self.font_height);
        write!(f, "rows: {}\n", self.rows);
        write!(f, "lines: {}\n", self.lines);
        write!(f, "font_widths: {:?}", self.widths)
    }
}
