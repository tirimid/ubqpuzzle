use nalgebra_glm::{Vec3, Mat4};
use nalgebra::geometry::UnitQuaternion;

pub type UnitQuat = UnitQuaternion<f32>;
pub struct Transform {
    pub pos: Vec3,
    pub rot: UnitQuat,
    pub scale: Vec3,
}

pub fn vec3_to_translation_mat(v: &Vec3) -> Mat4 {
    Mat4::new(
        1.0, 0.0, 0.0, v.x,
        0.0, 1.0, 0.0, v.y,
        0.0, 0.0, 1.0, v.z,
        0.0, 0.0, 0.0, 1.0,
    )
}

// cannot use nalgebra method in `UnitQuaternion` as resulting matrix is of size 3x3.
// for opengl purposes, this matrix must be of size 4x4.
pub fn quat_to_rotation_mat(q: &UnitQuat) -> Mat4 {
    let (w, x, y, z) = {
        let coords = q.quaternion().coords;
        (coords.w, coords.x, coords.y, coords.z)
    };
    Mat4::new(
        2.0 * (w.powi(2) + x.powi(2)) - 1.0,
        2.0 * (x * y - w * z),
        2.0 * (x * z + w * y),
        0.0,
        2.0 * (x * y + w * z),
        2.0 * (w.powi(2) + y.powi(2)) - 1.0,
        2.0 * (y * z - w * z),
        0.0,
        2.0 * (x * z - w * y),
        2.0 * (y * z + w * x),
        2.0 * (w.powi(2) + z.powi(2)) - 1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn vec3_to_scale_mat(v: &Vec3) -> Mat4 {
    Mat4::new(
        v.x, 0.0, 0.0, 0.0,
        0.0, v.y, 0.0, 0.0,
        0.0, 0.0, v.z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 0.0),
            rot: UnitQuat::identity(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn trans_mat(&self) -> Mat4 {
        let trans = vec3_to_translation_mat(&self.pos);
        let rot = quat_to_rotation_mat(&self.rot);
        let scale = vec3_to_scale_mat(&self.scale);
        trans * rot * scale
    }

    pub fn forward(&self) -> Vec3 {
        self.rot.to_rotation_matrix() * Vec3::new(0.0, 0.0, 1.0)
    }
}
