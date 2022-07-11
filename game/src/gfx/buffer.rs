use gl::types::{GLenum, GLuint, GLsizeiptr, GLvoid};
use std::mem;

pub struct Buffer {
    id: GLuint,
    len: usize,
    buf_type: GLenum,
}

impl Buffer {
    pub fn new<T>(data: &[T], usage: GLenum, buf_type: GLenum) -> Self {
        let id = unsafe {
            let mut id = 0;
            gl::GenBuffers(1, &mut id);
            id
        };

        unsafe {
            gl::BindBuffer(buf_type, id);
            gl::BufferData(
                buf_type,
                (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage,
            );
            Self::unbind(buf_type);
        }

        Self { id, len: data.len(), buf_type }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buf_type, self.id);
        }
    }

    pub fn unbind(buf_type: GLenum) {
        unsafe {
            gl::BindBuffer(buf_type, 0);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn buf_type(&self) -> GLenum {
        self.buf_type
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
