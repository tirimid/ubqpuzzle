use memoffset::offset_of;
use nalgebra_glm::Vec3;

pub struct Vertex {
    pub pos: Vec3,
    pub col: Vec3,
}

impl Vertex {
    pub fn pos_offset() -> usize {
        offset_of!(Self, pos)
    }

    pub fn col_offset() -> usize {
        offset_of!(Self, col)
    }
}
