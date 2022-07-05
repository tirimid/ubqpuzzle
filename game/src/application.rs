use crate::gfx::graphics::Graphics;
use sdl2::Sdl;
use sdl2::event::Event;

pub struct Application {
    sdl: Sdl,
    gfx: Graphics,
}

impl Application {
    pub fn new(wnd_title: &str, wnd_size: (u32, u32)) -> Self {
        let sdl = sdl2::init().unwrap();
        let gfx = Graphics::new(&sdl, wnd_title, wnd_size);
        
        Self { sdl, gfx }
    }

    pub fn main_loop(&self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main_loop,
                    _ => (),
                }
            }

            self.gfx.draw();
            self.gfx.present();
        }
    }
}
