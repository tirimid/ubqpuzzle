use sdl2::video::{GLContext, Window};
use sdl2::Sdl;
use std::os::raw::c_void;
use crate::gfx::program::Program;
use crate::gfx::camera::Camera;

pub struct Graphics {
    wnd: Window,
    _gl_ctx: GLContext,
    program: Program,
}

impl Graphics {
    pub fn new(sdl: &Sdl, wnd_title: &str, wnd_size: (u32, u32)) -> Self {
        let vid_subsys = sdl.video().unwrap();
        let wnd = vid_subsys.window(wnd_title, wnd_size.0, wnd_size.1).opengl().build().unwrap();
        let _gl_ctx = wnd.gl_create_context().unwrap();
        gl::load_with(|s| vid_subsys.gl_get_proc_address(s) as *const c_void);
        let program = Program::new(
            "assets/shaders/shader.vert",
            "assets/shaders/shader.frag",
        ).unwrap();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Self { wnd, _gl_ctx, program }
    }

    pub fn prepare(&self, cam: &Camera) {
        self.program.use_program();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        self.program.uniform_mat4_set(
            self.program.uniform_mat4_location("view_mat\0").unwrap(),
            &cam.view_mat(),
        );
        self.program.uniform_mat4_set(
            self.program.uniform_mat4_location("proj_mat\0").unwrap(),
            &cam.proj_mat(),
        );
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn present(&self) {
        self.wnd.gl_swap_window();
    }
}
