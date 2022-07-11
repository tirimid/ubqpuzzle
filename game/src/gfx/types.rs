use memoffset::offset_of;

pub struct Position(f32, f32, f32);

impl Position {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
}

pub struct Color(f32, f32, f32);

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b)
    }
}

pub struct Vertex {
    pos: Position,
    col: Color,
}

impl Vertex {
    pub const fn new(pos: Position, col: Color) -> Self {
        Self { pos, col }
    }
    
    pub fn pos_offset() -> usize {
        offset_of!(Self, pos)
    }

    pub fn col_offset() -> usize {
        offset_of!(Self, col)
    }
}
