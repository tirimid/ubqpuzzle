use gl::types::{GLenum, GLuint, GLsizeiptr, GLvoid};
use std::mem;

pub struct Buffer {
    id: GLuint,
    len: usize,
}

impl Buffer {
    pub fn new<T>(data: &Vec<T>, usage: GLenum) -> Self {
        let id = unsafe {
            let mut id = 0;
            gl::GenBuffers(1, &mut id);
            id
        };

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage,
            );
            Self::unbind();
        }

        Self { id, len: data.len() }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
