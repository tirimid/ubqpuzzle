use sdl2::video::{GLContext, Window};
use sdl2::Sdl;
use std::os::raw::c_void;
use crate::gfx::program::Program;
use crate::gfx::vao::Vao;
use crate::gfx::gmodels;

pub struct Graphics {
    wnd: Window,
    gl_ctx: GLContext,
    program: Program,
    vao: Vao,
}

impl Graphics {
    pub fn new(sdl: &Sdl, wnd_title: &str, wnd_size: (u32, u32)) -> Self {
        let vid_subsys = sdl.video().unwrap();
        let wnd = vid_subsys.window(wnd_title, wnd_size.0, wnd_size.1).opengl().build().unwrap();
        let gl_ctx = wnd.gl_create_context().unwrap();
        gl::load_with(|s| vid_subsys.gl_get_proc_address(s) as *const c_void);
        let program = Program::new(
            "assets/shaders/shader.vert",
            "assets/shaders/shader.frag",
        ).unwrap();

        let vao = Vao::new(&gmodels::QUAD_VERTS, &gmodels::QUAD_INDS);

        Self { wnd, gl_ctx, program, vao }
    }

    pub fn draw(&self) {
        self.program.use_program();
        self.vao.easy_draw();
    }

    pub fn present(&self) {
        self.wnd.gl_swap_window();
    }
}
