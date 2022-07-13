use crate::gfx::vao::Vao;
use crate::types::transform::Transform;
use crate::gfx::program::Program;

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

    pub fn draw(&self, prog: &Program) {
        prog.uniform_mat4_set(
            prog.uniform_mat4_location("model_mat\0").unwrap(),
            &self.trans.trans_mat(),
        );
        self.vao.draw();
    }
}
