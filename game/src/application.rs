use crate::gfx::graphics::Graphics;
use sdl2::Sdl;
use sdl2::event::Event;
use crate::gfx::camera::Camera;
use crate::gfx::model::Model;
use crate::gfx::vao::Vao;
use crate::gfx::gmodels;
use nalgebra_glm::Vec3;
use crate::types::transform::UnitQuat;

pub struct Application {
    sdl: Sdl,
    gfx: Graphics,
    cam: Camera,
    model0: Model,
    model1: Model,
}

impl Application {
    pub fn new(wnd_title: &str, wnd_size: (u32, u32)) -> Self {
        let sdl = sdl2::init().unwrap();
        let gfx = Graphics::new(&sdl, wnd_title, wnd_size);

        let cam = Camera::new((wnd_size.0 as f32, wnd_size.1 as f32));
        let mut model0 = Model::new(Vao::new(&**gmodels::TRIANGLE_VERTS, &gmodels::TRIANGLE_INDS));
        model0.trans.pos += Vec3::new(1.2, 0.7, 1.2);
        let mut model1 = Model::new(Vao::new(&**gmodels::CUBE_VERTS, &gmodels::CUBE_INDS));
        model1.trans.pos += Vec3::new(-1.2, 0.0, -1.2);
        
        Self { sdl, gfx, cam, model0, model1 }
    }

    pub fn main_loop(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main_loop,
                    _ => (),
                }
            }

            self.cam.trans.rot *= UnitQuat::from_euler_angles(0.0, 0.007, 0.0);
            self.model0.trans.rot *= UnitQuat::from_euler_angles(0.0, 0.01, 0.0);
            self.model1.trans.rot *= UnitQuat::from_euler_angles(0.0, 0.01, 0.01);

            self.gfx.prepare(&self.cam);
            self.model0.draw(&self.gfx.program());
            self.model1.draw(&self.gfx.program());
            self.gfx.present();
        }
    }
}
