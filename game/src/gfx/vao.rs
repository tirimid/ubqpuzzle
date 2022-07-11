use gl::types::{GLint, GLuint, GLvoid};
use crate::gfx::buffer::Buffer;
use crate::gfx::types::Vertex;
use std::mem;
use std::ptr;

pub struct Vao {
    id: GLuint,
    vbo: Buffer,
    ibo: Buffer,
}

impl Vao {
    pub fn new(verts: &[Vertex], inds: &[u32]) -> Self {
        let vbo = Buffer::new(verts, gl::STATIC_DRAW, gl::ARRAY_BUFFER);
        let ibo = Buffer::new(inds, gl::STATIC_DRAW, gl::ELEMENT_ARRAY_BUFFER);
        let id = unsafe {
            let mut id = 0;
            gl::GenVertexArrays(1, &mut id);
            id
        };

        unsafe {
            gl::BindVertexArray(id);
            vbo.bind();

            // position attribute
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLint,
                Vertex::pos_offset() as *const GLvoid,
            );

            // color attribute
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as GLint,
                Vertex::col_offset() as *const GLvoid,
            );
            
            Buffer::unbind(gl::ARRAY_BUFFER);
            Self::unbind();
        }

        Self { id, vbo, ibo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            self.ibo.bind();
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.ibo.len() as GLint,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    // handles binding and unbinding buffers.
    // use this function wherever possible to avoid possible problems.
    pub fn easy_draw(&self) {
        self.bind();
        self.draw();
        Self::unbind();
        Buffer::unbind(self.ibo.buf_type());
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
