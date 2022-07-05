use gl::types::{GLint, GLuint, GLvoid};
use crate::gfx::buffer::Buffer;
use crate::gfx::types::Vertex;
use std::mem;

pub struct Vao {
    id: GLuint,
    vbo: Buffer,
}

impl Vao {
    pub fn new(verts: &Vec<Vertex>) -> Self {
        let vbo = Buffer::new(verts, gl::STATIC_DRAW);
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
            
            Buffer::unbind();
            Self::unbind();
        }

        Self { id, vbo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vbo.len() as GLint);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
