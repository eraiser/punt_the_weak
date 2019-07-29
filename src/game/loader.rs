pub fn load_texture(path: &str) -> u32 {
    let mut texture = 0;

    use std::path::Path;
    use crate::image::GenericImageView;

    let im = image::open(&Path::new(path)).unwrap().flipv();

    let im = match im {
        image::DynamicImage::ImageRgba8(img) => img,
        img => img.to_rgba()
    };


    let dim = im.dimensions();

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, dim.0 as i32, dim.1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE,
                       im.into_raw().as_ptr() as *mut std::ffi::c_void );

        //std::mem::transmute(&im.raw_pixels()[0])
        // ... nice trilinear filtering ...
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        // ... which requires mipmaps. Generate them automatically.
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    texture
}

use crate::game::model::mesh::Mesh;
use crate::game::model::mesh;


pub fn load_collada_mesh(file: &str) -> Mesh {
    let doc = collada::document::ColladaDocument::from_str(file).unwrap();

    let obj_data = &doc.get_obj_set().unwrap().objects[0];

    let vertices_indexed = &obj_data.vertices;
    let texture_data_indexed = &obj_data.tex_vertices;
    let normals_indexed = &obj_data.normals;

    let prim_el = &obj_data.geometry[0].mesh[0];


    let indices = match prim_el {
        collada::PrimitiveElement::Triangles(t) => t,
        _ => {
            panic!();
        }
    };

    let (vert,
        tex,
        norm,
        int) = fill_vtn_vectors(vertices_indexed
                                , texture_data_indexed
                                , normals_indexed
                                , indices);


    mesh::new_static_3d_mesh(vert, norm, tex, int)
}

use collada::{Vertex, TVertex, Triangles};

fn fill_vtn_vectors(vertices_indexed: &Vec<Vertex>, texture_data_indexed: &Vec<TVertex>, normals_indexed: &Vec<Vertex>, indices: &Triangles)
                    -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<i16>) {
    let mut vert = Vec::new();
    let mut tex = Vec::new();
    let mut norm = Vec::new();

    let mut int = Vec::new();
    let mut i = 0;

    let ind_i = indices.vertices.iter();


    for triangle in indices.vertices.iter() {
        let vtn_i = triangle.0;
        let v_i_0 = vtn_i.0;
        let t_i_0 = vtn_i.1.unwrap();
        let n_i_0 = vtn_i.2.unwrap();

        let vtn_i = triangle.1;
        let v_i_1 = vtn_i.0;
        let t_i_1 = vtn_i.1.unwrap();
        let n_i_1 = vtn_i.2.unwrap();

        let vtn_i = triangle.2;
        let v_i_2 = vtn_i.0;
        let t_i_2 = vtn_i.1.unwrap();
        let n_i_2 = vtn_i.2.unwrap();

        vert.push(vertices_indexed[v_i_0].x as f32);
        vert.push(vertices_indexed[v_i_0].y as f32);
        vert.push(vertices_indexed[v_i_0].z as f32);
        tex.push(texture_data_indexed[t_i_0].x as f32);
        tex.push(texture_data_indexed[t_i_0].y as f32);
        norm.push(normals_indexed[n_i_0].x as f32);
        norm.push(normals_indexed[n_i_0].y as f32);
        norm.push(normals_indexed[n_i_0].z as f32);

        vert.push(vertices_indexed[v_i_1].x as f32);
        vert.push(vertices_indexed[v_i_1].y as f32);
        vert.push(vertices_indexed[v_i_1].z as f32);
        tex.push(texture_data_indexed[t_i_1].x as f32);
        tex.push(texture_data_indexed[t_i_1].y as f32);
        norm.push(normals_indexed[n_i_1].x as f32);
        norm.push(normals_indexed[n_i_1].y as f32);
        norm.push(normals_indexed[n_i_1].z as f32);

        vert.push(vertices_indexed[v_i_2].x as f32);
        vert.push(vertices_indexed[v_i_2].y as f32);
        vert.push(vertices_indexed[v_i_2].z as f32);
        tex.push(texture_data_indexed[t_i_2].x as f32);
        tex.push(texture_data_indexed[t_i_2].y as f32);
        norm.push(normals_indexed[n_i_2].x as f32);
        norm.push(normals_indexed[n_i_2].y as f32);
        norm.push(normals_indexed[n_i_2].z as f32);


        int.push(i);
        int.push(i + 1);
        int.push(i + 2);
        i += 3;
    }

    (vert, tex, norm, int)
}


