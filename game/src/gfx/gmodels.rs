#![allow(dead_code)]

use crate::gfx::types::Vertex;
use lazy_static::lazy_static;
use nalgebra_glm::Vec3;

// a bunch of generic models.
// for testing purposes, vertices and indices are seperated.
lazy_static! {
    pub static ref TRIANGLE_VERTS: Box<[Vertex; 3]> = Box::new([
        Vertex { pos: Vec3::new(0.0, 0.5, 0.0), col: Vec3::new(1.0, 0.0, 0.0) },
        Vertex { pos: Vec3::new(0.5, -0.5, 0.0), col: Vec3::new(0.0, 1.0, 0.0) },
        Vertex { pos: Vec3::new(-0.5, -0.5, 0.0), col: Vec3::new(0.0, 0.0, 1.0) },
    ]);
    
    pub static ref QUAD_VERTS: Box<[Vertex; 4]> = Box::new([
        Vertex { pos: Vec3::new(0.5, 0.5, 0.0), col: Vec3::new(1.0, 0.0, 0.0) },
        Vertex { pos: Vec3::new(0.5, -0.5, 0.0), col: Vec3::new(0.0, 1.0, 0.0) },
        Vertex { pos: Vec3::new(-0.5, -0.5, 0.0), col: Vec3::new(0.0, 0.0, 1.0) },
        Vertex { pos: Vec3::new(-0.5, 0.5, 0.0), col: Vec3::new(0.0, 1.0, 0.0) },
    ]);

    pub static ref CUBE_VERTS: Box<[Vertex; 8]> = Box::new([
        Vertex { pos: Vec3::new(0.5, 0.5, -0.5), col: Vec3::new(1.0, 0.0, 0.0) },
        Vertex { pos: Vec3::new(-0.5, 0.5, -0.5), col: Vec3::new(0.0, 1.0, 0.0) },
        Vertex { pos: Vec3::new(-0.5, -0.5, -0.5), col: Vec3::new(0.0, 0.0, 1.0) },
        Vertex { pos: Vec3::new(0.5, -0.5, -0.5), col: Vec3::new(1.0, 1.0, 1.0) },
        Vertex { pos: Vec3::new(0.5, 0.5, 0.5), col: Vec3::new(1.0, 0.0, 0.0) },
        Vertex { pos: Vec3::new(-0.5, 0.5, 0.5), col: Vec3::new(0.0, 1.0, 0.0) },
        Vertex { pos: Vec3::new(-0.5, -0.5, 0.5), col: Vec3::new(0.0, 0.0, 1.0) },
        Vertex { pos: Vec3::new(0.5, -0.5, 0.5), col: Vec3::new(1.0, 1.0, 1.0) },
    ]);
}

pub const TRIANGLE_INDS: [u32; 3] = [0, 1, 2];
pub const QUAD_INDS: [u32; 6] = [0, 1, 2, 0, 2, 3];
pub const CUBE_INDS: [u32; 36] = [
    0, 1, 2, 0, 2, 3, // front.
    1, 5, 6, 1, 6, 2, // right.
    5, 4, 7, 5, 7, 6, // back.
    4, 0, 3, 4, 3, 7, // left.
    4, 5, 1, 4, 1, 0, // top.
    3, 2, 6, 3, 6, 7, // bottom.
];
