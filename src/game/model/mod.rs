mod mesh;

pub struct Model{
    mesh: mesh::Mesh
}

impl Model{
    pub fn draw_2d(&self) {
        self.mesh.draw_2d();
    }
    pub fn cleanup(&self) {
        self.mesh.cleanup();
    }
}


use gl::types::*;



// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

pub fn triangle() -> Model{
    Model{
        mesh: mesh::new_static_2d_mesh(VERTEX_DATA.to_vec())
    }
}
