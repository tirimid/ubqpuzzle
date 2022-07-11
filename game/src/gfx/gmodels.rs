#![allow(dead_code)]

use crate::gfx::types::{Position, Color, Vertex};

// a bunch of generic models.
// for testing purposes, vertices and indices are seperated.
pub const TRIANGLE_VERTS: [Vertex; 3] = [
    Vertex::new(Position::new(0.0, 0.5, 0.0), Color::new(1.0, 0.0, 0.0)),
    Vertex::new(Position::new(0.5, -0.5, 0.0), Color::new(0.0, 1.0, 0.0)),
    Vertex::new(Position::new(-0.5, -0.5, 0.0), Color::new(0.0, 0.0, 1.0)),
];

pub const QUAD_VERTS: [Vertex; 4] = [
    Vertex::new(Position::new(-0.5, 0.5, 0.0), Color::new(1.0, 0.0, 0.0)),
    Vertex::new(Position::new(0.5, 0.5, 0.0), Color::new(0.0, 1.0, 0.0)),
    Vertex::new(Position::new(0.5, -0.5, 0.0), Color::new(0.0, 0.0, 1.0)),
    Vertex::new(Position::new(-0.5, -0.5, 0.0), Color::new(0.0, 1.0, 0.0)),
];

pub const TRIANGLE_INDS: [u32; 3] = [0, 1, 2];
pub const QUAD_INDS: [u32; 6] = [0, 1, 2, 0, 2, 3];
