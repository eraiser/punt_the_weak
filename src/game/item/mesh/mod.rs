#[derive(Clone)]
pub struct Mesh {
    texture: gl::types::GLuint,
    vertex_array_id: gl::types::GLuint,
    vertex_buffer_id: gl::types::GLuint,
    uv_buffer_id: gl::types::GLuint,
    normal_buffer_id: gl::types::GLuint,
    element_buffer_id: gl::types::GLuint,
    vertex_count: i32,
}

use gl::types::*;
use std::ptr;

pub fn new_untextured_mesh(
    vertex_data: Vec<f32>,
    normal_data: Vec<f32>,
    uv_data: Vec<f32>,
    indices: Vec<i16>,
) -> Mesh {
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

    Mesh {
        texture: 0,
        vertex_array_id,
        vertex_buffer_id,
        uv_buffer_id,
        normal_buffer_id,
        element_buffer_id,
        vertex_count,
    }
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

impl Mesh {
    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.vertex_count,
                gl::UNSIGNED_SHORT,
                ptr::null(),
            );
        }
    }

    pub fn enable_buffers(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_buffer_id);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer_id);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_id);
        }
    }

    pub fn disable_buffers(&self) {
        unsafe {
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);
        }
    }

    pub fn set_texture(&mut self, t: u32) {
        self.texture = t;
    }
    pub fn get_texture(&self) -> u32 {
        self.texture
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vertex_buffer_id);
            gl::DeleteBuffers(1, &self.normal_buffer_id);
            gl::DeleteBuffers(1, &self.uv_buffer_id);
            gl::DeleteVertexArrays(1, &self.vertex_array_id);

            gl::DeleteTextures(1, &self.texture);
        }
    }
}
