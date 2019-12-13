use cgmath::{Matrix4, Vector3};
use gl::types::*;

use crate::settings::MAX_LIGHTS;

pub struct Renderer {
    perspective_matrix: cgmath::Matrix4<f32>,
    shader_program_3d: GLuint,
    shader_program_2d: GLuint,
    light_positions_location: GLint,
    light_colors_location: GLint,
    light_powers_location: GLint,
    m_location: GLint,
    v_location: GLint,
    mvp_location: GLint,
    texture_location: GLint,
}

impl Renderer {
    pub fn use_3d_program(&self) {
        unsafe {
            gl::UseProgram(self.shader_program_3d);
        }
    }
    pub fn use_2d_program(&self) {
        unsafe {
            gl::UseProgram(self.shader_program_2d);
        }
    }
    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteProgram(self.shader_program_3d);
            gl::DeleteProgram(self.shader_program_2d);
        }
    }
    pub fn set_uniform_mvp(&self, m_matrix: Matrix4<f32>, v_matrix: Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.mvp_location,
                1,
                gl::FALSE,
                cgmath::conv::array4x4(self.perspective_matrix * v_matrix * m_matrix).as_ptr()
                    as *const GLfloat,
            )
        }
    }
    pub fn set_uniform_v(&self, m_matrix: Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.v_location,
                1,
                gl::FALSE,
                cgmath::conv::array4x4(m_matrix).as_ptr() as *const GLfloat,
            )
        }
    }
    pub fn set_uniform_m(&self, v_matrix: Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.m_location,
                1,
                gl::FALSE,
                cgmath::conv::array4x4(v_matrix).as_ptr() as *const GLfloat,
            )
        }
    }
    pub fn set_uniform_light_positions_worldspace(
        &self,
        light_positions: [Vector3<f32>; MAX_LIGHTS],
    ) {
        unsafe {
            gl::Uniform3fv(
                self.light_positions_location,
                MAX_LIGHTS as i32,
                light_positions.as_ptr() as *const GLfloat,
            )
        }
    }
    pub fn set_uniform_light_colors(&self, light_colors: [Vector3<f32>; MAX_LIGHTS]) {
        unsafe {
            gl::Uniform3fv(
                self.light_colors_location,
                MAX_LIGHTS as i32,
                light_colors.as_ptr() as *const GLfloat,
            )
        }
    }
    pub fn set_uniform_light_powers(&self, light_powers: [f32; MAX_LIGHTS]) {
        unsafe {
            gl::Uniform1fv(
                self.light_powers_location,
                MAX_LIGHTS as i32,
                light_powers.as_ptr() as *const GLfloat,
            )
        }
    }
    pub fn set_texture(&self, texture: GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::Uniform1i(self.texture_location, 0);
        }
    }
}

mod shader_utilities;

pub fn init_renderer() -> Renderer {

    let vs_3d = shader_utilities::compile_shader(
        include_str!("shader/StandardVertShading.glsl"),
        gl::VERTEX_SHADER,
    );
    let fs_3d = shader_utilities::compile_shader(
        include_str!("shader/StandardFragShading.glsl"),
        gl::FRAGMENT_SHADER,
    );

    let shader_program_3d = shader_utilities::link_program(vs_3d, fs_3d);

    unsafe {
        gl::DeleteShader(fs_3d);
        gl::DeleteShader(vs_3d);
    }

    let vs_2d = shader_utilities::compile_shader(
        include_str!("shader/VertexShader2d.glsl"),
        gl::VERTEX_SHADER,
    );
    let fs_2d = shader_utilities::compile_shader(
        include_str!("shader/FragmentShader2d.glsl"),
        gl::FRAGMENT_SHADER,
    );

    let shader_program_2d = shader_utilities::link_program(vs_2d, fs_2d);

    unsafe {
        gl::DeleteShader(fs_2d);
        gl::DeleteShader(vs_2d);
    }

    let perspective_matrix: Matrix4<f32> = {
        cgmath::perspective(
            cgmath::Rad(std::f32::consts::PI / 2.1),
            1920.0 / 1080.0,
            0.1,
            1000.0,
        )
    };

    use std::ffi::CString;
    let mvp_location: GLint =
        unsafe { gl::GetUniformLocation(shader_program_3d, CString::new("MVP").unwrap().as_ptr()) };
    let v_location: GLint =
        unsafe { gl::GetUniformLocation(shader_program_3d, CString::new("V").unwrap().as_ptr()) };
    let m_location: GLint =
        unsafe { gl::GetUniformLocation(shader_program_3d, CString::new("M").unwrap().as_ptr()) };
    let light_positions_location: GLint = unsafe {
        gl::GetUniformLocation(
            shader_program_3d,
            CString::new("LightPositions_worldspace").unwrap().as_ptr(),
        )
    };
    let light_colors_location: GLint = unsafe {
        gl::GetUniformLocation(
            shader_program_3d,
            CString::new("LightColors").unwrap().as_ptr(),
        )
    };
    let light_powers_location: GLint = unsafe {
        gl::GetUniformLocation(
            shader_program_3d,
            CString::new("LightPowers").unwrap().as_ptr(),
        )
    };
    let texture_location: GLint = unsafe {
        gl::GetUniformLocation(
            shader_program_3d,
            CString::new("TextureSampler").unwrap().as_ptr(),
        )
    };

    Renderer {
        shader_program_3d,
        perspective_matrix,
        light_positions_location,
        light_colors_location,
        light_powers_location,
        m_location,
        v_location,
        mvp_location,
        texture_location,
        shader_program_2d
    }
}
