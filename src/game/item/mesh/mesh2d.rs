use std::ptr;

use gl::types::*;

pub struct Mesh2D {
    pub texture: gl::types::GLuint,
    vertex_buffer_id: gl::types::GLuint,
    uv_buffer_id: gl::types::GLuint,
    vertex_count: i32
}

pub fn new_texture_2d(
    texture: gl::types::GLuint,
    vertex_buffer_id: gl::types::GLuint,
    uv_buffer_id: gl::types::GLuint,
    vertex_count: i32)  -> Mesh2D {
    Mesh2D {
        texture,
        vertex_buffer_id,
        uv_buffer_id,
        vertex_count
    }
}

impl Mesh2D {

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                self.vertex_count,
            );
        }
    }

    pub fn enable_buffers(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer_id);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
        }
    }

    pub fn disable_buffers(&self) {
        unsafe {
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
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
            gl::DeleteBuffers(1, &self.uv_buffer_id);
            gl::DeleteTextures(1, &self.texture);
        }
    }
}