use gl::types::*;

pub struct Renderer {
    shader_program_2d: GLuint
}

impl Renderer {
    pub fn use_2d_program(&self) {
        unsafe {
            gl::UseProgram(self.shader_program_2d);
        }
    }
    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteProgram(self.shader_program_2d);
        }
    }
}

mod shader_utilities;

pub fn init_renderer() -> Renderer {
    let vs = shader_utilities::compile_shader(include_str!("../shader/vs.glsl"), gl::VERTEX_SHADER);
    let fs = shader_utilities::compile_shader(include_str!("../shader/fs.glsl"), gl::FRAGMENT_SHADER);
    let program = shader_utilities::link_program(vs, fs);
    unsafe {
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
    }
    Renderer {
        shader_program_2d: program
    }
}