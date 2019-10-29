mod model;
mod mesh;
pub mod lighting;

mod loader;

use std::collections::HashMap;
use cgmath::Vector3;

use crate::settings::MAX_LIGHTS;
use rayon::prelude::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

/*
model_sets is a Vector that represents models with the same mesh and texture
that only differ in there transformations and/or motions

model_map is a HashMap that gets a new elements every time a new mesh
that hasn't been loaded is loaded and keys the index to the loaded element to
an element in model_sets. The key is the file path.
*/

pub struct ItemHandler {
    pub model_sets: Vec<(Vec<model::ModelTransforms>, mesh::Mesh)>,
    model_map: HashMap<String, usize>,
    pub light_sorces: Vec<lighting::LightSource>,
}

pub fn new_item_handler() -> ItemHandler {
    ItemHandler {
        model_sets: Vec::new(),
        model_map: HashMap::new(),
        light_sorces: Vec::new(),
    }
}

impl ItemHandler {
    pub fn add_new_model(&mut self, collada_path: &str, image_path: &str)
                         -> &mut model::ModelTransforms {
        let contains = self.model_map.get(collada_path);


        let i = match contains {
            Some(x) => *x,
            None => {
                let transform_vec = Vec::new();

                let mut mesh = loader::load_collada_mesh(collada_path);
                let texture = loader::load_texture(image_path);
                mesh.set_texture(texture);

                println!("loading");

                self.model_sets.push((transform_vec, mesh));

                self.model_map.insert(collada_path.to_string(), self.model_sets.len() - 1);
                self.model_sets.len() - 1
            }
        };

        let new_transform = model::new_model_transform();
        self.model_sets[i].0.push(new_transform);

        let y = self.model_sets[i].0.len() - 1;

        &mut self.model_sets[i].0[y]
    }

    pub fn add_light_source(&mut self, light: lighting::LightSource) {
        self.light_sorces.push(light);
    }

    pub fn get_nearest_light_data(&self) -> ([Vector3<f32>; MAX_LIGHTS], [Vector3<f32>; MAX_LIGHTS], [f32; MAX_LIGHTS]) {

        /*
        TODO: Actually calculate closest 4 light sources
        */

        let z:Vector3<f32> = Vector3{x:0.0,y:0.0,z:0.0};
        let mut pos: [Vector3<f32>; MAX_LIGHTS] = [z,z,z,z];
        let mut col: [Vector3<f32>; MAX_LIGHTS] = [z,z,z,z];
        let mut pow: [f32; MAX_LIGHTS] = [0.0,0.0,0.0,0.0];

        for i in 0..MAX_LIGHTS {
            pos[i] = self.light_sorces[i].translation;
            col[i] = self.light_sorces[i].color;
            pow[i] = self.light_sorces[i].power;
        }

        return (pos, col, pow);
    }

    pub fn update(&mut self) {
        self.model_sets.par_iter_mut().for_each({ |s|
                s.0.par_iter_mut().for_each(|m| m.update())
        });
    }

    pub fn calc_intp_modelmatrices(&mut self, i_v: f32) {
        self.model_sets.par_iter_mut().for_each( {|s|
            s.0.par_iter_mut().for_each(|m| m.calc_intp_model_matrix(i_v))
        })
    }

    pub fn cleanup(&self) {
        self.model_sets.iter()
            .for_each(|e| e.1.cleanup());
    }
}