use crate::gfx::graphics::Graphics;
use sdl2::Sdl;
use sdl2::event::Event;
use crate::gfx::camera::Camera;
use crate::gfx::model::Model;

pub struct Application {
    sdl: Sdl,
    gfx: Graphics,
    cam: Camera,
    mdl: Model,
}

impl Application {
    pub fn new(wnd_title: &str, wnd_size: (u32, u32)) -> Self {
        let sdl = sdl2::init().unwrap();
        let gfx = Graphics::new(&sdl, wnd_title, wnd_size);

        let cam = Camera::new((wnd_size.0 as f32, wnd_size.1 as f32));
        let mut mdl = Model::from_obj_file("assets/models/cube.obj").unwrap();
        mdl.trans.move_pos(0.0, 0.0, -3.0);
        
        Self { sdl, gfx, cam, mdl }
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

            self.mdl.trans.rotate(0.005, 0.005, 0.01);

            self.gfx.prepare(&self.cam);
            self.mdl.draw(&self.gfx.program());
            self.gfx.present();
        }
    }
}
