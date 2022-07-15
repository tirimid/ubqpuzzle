use crate::types::transform::Transform;
use nalgebra_glm::Mat4;

pub struct Camera {
    pub trans: Transform,
    depth_clip: (f32, f32), // (near, far).
    fov: f32,
    aspect_ratio: f32,
}

impl Camera {
    pub fn new(wnd_res: (f32, f32)) -> Self {
        Self {
            trans: Transform::identity(),
            depth_clip: (0.1, 1000.0),
            fov: 2.0, // in radians.
            aspect_ratio: wnd_res.0 / wnd_res.1,
        }
    }

    pub fn view_mat(&self) -> Mat4 {
        self.trans.trans_mat()
    }

    pub fn proj_mat(&self) -> Mat4 {
        nalgebra_glm::perspective(self.aspect_ratio, self.fov, self.depth_clip.0, self.depth_clip.1)
    }
}
