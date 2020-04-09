use std::ptr;

use gl::types::*;

use mesh3d::Mesh3D;

use crate::game::item::loader::load_texture;


pub mod mesh3d;
pub mod mesh2d;
mod fonts;

pub fn new_untextured_mesh(
    vertex_data: Vec<f32>,
    normal_data: Vec<f32>,
    uv_data: Vec<f32>,
    indices: Vec<i16>,
) -> Mesh3D {
    let mut vertex_array_id = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }

    let vertex_buffer_id = fill_buffer(vertex_data);
    let normal_buffer_id = fill_buffer(normal_data);
    let uv_buffer_id = fill_buffer(uv_data);

    let mut element_buffer_id = 0;
    unsafe {
        gl::GenBuffers(1, &mut element_buffer_id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_id);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<GLshort>()) as GLsizeiptr,
            std::mem::transmute(&indices[0]),
            gl::STATIC_DRAW,
        );
    }
    let vertex_count = indices.len() as i32;
    mesh3d::new_mesh_3d(vertex_array_id,vertex_buffer_id,uv_buffer_id,normal_buffer_id,element_buffer_id,vertex_count)
}

pub fn new_2d_text(string: &str) -> mesh2d::Mesh2D {
    let font = fonts::FontData::load_font();
    let (vert, uvs) = font.generate_2d_text_vert_uv_data(string);

    let vertex_count: i32 = vert.len() as i32 / 2;

    let vertex_buffer_id = fill_buffer(vert);
    let uv_buffer_id = fill_buffer(uvs);

    let texture_id = load_texture("./res/fonts/ExportedFont.tga");

    mesh2d::new_texture_2d(texture_id, vertex_buffer_id, uv_buffer_id, vertex_count)
}

fn fill_buffer(buffer_data: Vec<f32>) -> u32 {
    let mut buffer_id = 0;

    unsafe {
        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut buffer_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (buffer_data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&buffer_data[0]),
            gl::STATIC_DRAW,
        );
    }

    buffer_id
}

