use gl::types::*;
use cgmath::Matrix4;

pub struct Renderer {
    shader_program_2d: GLuint,
    shader_program_3d: GLuint,
    perspective_matrix: cgmath::Matrix4<f32>,
    mvp_location: GLint
}

impl Renderer {
    pub fn use_2d_program(&self) {
        unsafe {
            gl::UseProgram(self.shader_program_2d);
        }
    }
    pub fn use_3d_program(&self) {
        unsafe {
            gl::UseProgram(self.shader_program_3d);
        }
    }
    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteProgram(self.shader_program_2d);
        }
    }
    pub fn set_mvp(&self,m_matrix: Matrix4<f32>,v_matrix: Matrix4<f32>)
    {
        unsafe {
            gl::UniformMatrix4fv(self.mvp_location, 1, gl::FALSE, cgmath::conv::array4x4(self.perspective_matrix*v_matrix*m_matrix).as_ptr() as *const GLfloat)
        }
    }
}

mod shader_utilities;

pub fn init_renderer() -> Renderer {

    let vs_2d = shader_utilities::compile_shader(include_str!("../shader/vs_2d.glsl"), gl::VERTEX_SHADER);
    let vs_3d = shader_utilities::compile_shader(include_str!("../shader/vs_3d.glsl"), gl::VERTEX_SHADER);

    let fs = shader_utilities::compile_shader(include_str!("../shader/fs.glsl"), gl::FRAGMENT_SHADER);

    let program_2d = shader_utilities::link_program(vs_2d, fs);
    let program_3d = shader_utilities::link_program(vs_3d, fs);

    unsafe {
        gl::DeleteShader(fs);
        gl::DeleteShader(vs_2d);
        gl::DeleteShader(vs_3d);
    }

    let perspective_matrix:Matrix4<f32> = {
            cgmath::perspective(
                cgmath::Rad(std::f32::consts::PI/2.1),
                1920.0/1080.0,
                0.1,
                1000.0,
            )
    };

    use std::ffi::CString;
    let mvp_location:GLint =
        unsafe{
            gl::GetUniformLocation(program_3d,CString::new("MVP").unwrap().as_ptr())
        };

    Renderer {
        shader_program_2d: program_2d,
        shader_program_3d: program_3d,
        perspective_matrix,
        mvp_location
    }
}