use nalgebra_glm::{Vec3, Mat4};
use nalgebra::geometry::UnitQuaternion;

// using `UnitQuaternion` instead of `Quaternion` as it has a nicer api for needed conversions.
// also, it is more accurate to use unit quaternions for rotation.
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

pub fn quat_to_rotation_mat(q: &UnitQuat) -> Mat4 {
    nalgebra_glm::mat3_to_mat4(&q.to_rotation_matrix().matrix())
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

    pub fn move_pos(&mut self, x: f32, y: f32, z: f32) {
        self.pos += Vec3::new(x, y, z);
    }

    // input is expected as euler angles.
    pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.rot *= UnitQuat::from_euler_angles(pitch, yaw, roll);
    }
}
