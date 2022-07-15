use crate::gfx::vao::Vao;
use crate::types::transform::Transform;
use crate::gfx::program::Program;
use std::path::Path;
use obj::{Obj, ObjError};
use std::fs::File;
use std::io::BufReader;
use crate::types::vertex::Vertex;
use nalgebra_glm::Vec3;

pub struct Model {
    pub trans: Transform,
    vao: Vao,
}

impl Model {
    pub fn new(vao: Vao) -> Self {
        Self {
            trans: Transform::identity(),
            vao,
        }
    }

    pub fn from_obj_file<P: AsRef<Path>>(path: P) -> Result<Self, ObjError> {
        let reader = BufReader::new(File::open(path)?);
        let obj: Obj<obj::Vertex, u32> = obj::load_obj(reader)?;
        let verts = {
            let mut verts = Vec::new();
            for v in obj.vertices {
                verts.push(Vertex {
                    pos: nalgebra_glm::make_vec3(&v.position),
                    col: Vec3::new(1.0, 0.0, 1.0),
                    norm: nalgebra_glm::make_vec3(&v.normal),
                });
            }
            verts
        };
        Ok(Self {
            trans: Transform::identity(),
            vao: Vao::new(verts.as_slice(), obj.indices.as_slice()),
        })
    }
    
    pub fn draw(&self, prog: &Program) {
        prog.uniform_mat4_set(
            prog.uniform_mat4_location("model_mat\0").unwrap(),
            &self.trans.trans_mat(),
        );
        self.vao.draw();
    }
}
