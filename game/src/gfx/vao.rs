use gl::types::{GLint, GLuint, GLvoid, GLenum};
use crate::gfx::buffer::Buffer;
use crate::types::vertex::Vertex;
use std::mem;
use std::ptr;

pub struct Vao {
    id: GLuint,
    _vbo: Buffer,
    ibo: Buffer,
}

fn set_vert_attr(attr_ind: GLuint, size: GLint, attr_type: GLenum, offset: usize) {
    unsafe {
        gl::EnableVertexAttribArray(attr_ind);
        gl::VertexAttribPointer(
            attr_ind,
            size,
            attr_type,
            gl::FALSE,
            mem::size_of::<Vertex>() as GLint,
            offset as *const GLvoid,
        )
    }
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
        }
        set_vert_attr(0, 3, gl::FLOAT, Vertex::pos_offset());
        set_vert_attr(1, 3, gl::FLOAT, Vertex::col_offset());
        set_vert_attr(2, 3, gl::FLOAT, Vertex::norm_offset());
        unsafe {
            Buffer::unbind(vbo.buf_type());
            gl::BindVertexArray(0);
        }

        Self { id, _vbo: vbo, ibo }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        self.ibo.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.ibo.len() as GLint,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            gl::BindVertexArray(0);
        }
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
