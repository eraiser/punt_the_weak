pub struct Mesh {
    vertex_array_id: gl::types::GLuint,
    vertex_buffer_id: gl::types::GLuint,
    vertex_count: i32,
}

use std::mem;
use std::ptr;
use gl::types::*;

pub fn new_static_2d_mesh(vertex_data: Vec<f32>) -> Mesh {


    let mut vao = 0;
    let mut vbo = 0;
    let vc = vertex_data.len();

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vc * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&vertex_data[0]),
            gl::STATIC_DRAW,
        );
    }

    Mesh {
        vertex_array_id: vao,
        vertex_buffer_id: vbo,
        vertex_count: vc as i32/2,
    }
}

pub fn new_static_3d_mesh(vertex_data: Vec<f32>) -> Mesh {
    let mut vao = 0;
    let mut vbo = 0;
    let vc = vertex_data.len();

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vc * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&vertex_data[0]),
            gl::STATIC_DRAW,
        );
    }

    Mesh {
        vertex_array_id: vao,
        vertex_buffer_id: vbo,
        vertex_count: vc as i32/3,
    }
}

impl Mesh {
    pub fn draw_2d(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_id);
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );

            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count );

            gl::DisableVertexAttribArray(0);
        }
    }
    pub fn draw_3d(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_id);
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );

            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count );

            gl::DisableVertexAttribArray(0);
        }
    }
    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vertex_buffer_id);
            gl::DeleteVertexArrays(1, &self.vertex_array_id);
        }

    }
}