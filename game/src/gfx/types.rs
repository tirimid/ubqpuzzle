use memoffset::offset_of;
use lazy_static::lazy_static;

pub struct Position(f32, f32, f32);
pub struct Color(f32, f32, f32);

pub struct Vertex {
    pos: Position,
    col: Color,
}

impl Vertex {
    pub fn pos_offset() -> usize {
        offset_of!(Self, pos)
    }

    pub fn col_offset() -> usize {
        offset_of!(Self, col)
    }
}

// vectors of vertices for debugging purposes
lazy_static! {
    pub static ref TRIANGLE: Vec<Vertex> = vec![
        Vertex { pos: Position(0.0, 0.5, 0.0), col: Color(1.0, 0.0, 0.0) },
        Vertex { pos: Position(0.5, -0.5, 0.0), col: Color(1.0, 0.0, 0.0) },
        Vertex { pos: Position(-0.5, -0.5, 0.0), col: Color(1.0, 0.0, 0.0) },
    ];
}
