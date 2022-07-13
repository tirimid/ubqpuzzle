use gl::types::{GLchar, GLuint, GLint, GLfloat};
use crate::gfx::shader::Shader;
use std::path::Path;
use std::ffi::CString;
use std::ptr;
use std::fmt;
use std::error::Error;
use nalgebra_glm::Mat4;

pub struct Program {
    id: GLuint,
    vert_shader: Shader,
    frag_shader: Shader,
}

#[derive(Debug)]
pub enum ProgramError {
    CreationFailed(String),
}

impl Error for ProgramError {}
impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Program {
    pub fn new<P: AsRef<Path>>(
        vert_shader_path: P,
        frag_shader_path: P,
    ) -> Result<Self, Box<dyn Error>> {
        let vert_shader = Shader::from_file(vert_shader_path, gl::VERTEX_SHADER)?;
        let frag_shader = Shader::from_file(frag_shader_path, gl::FRAGMENT_SHADER)?;
        let id = unsafe {
            gl::CreateProgram()
        };
        
        unsafe {
            gl::AttachShader(id, vert_shader.id());
            gl::AttachShader(id, frag_shader.id());
            gl::LinkProgram(id);
        }

        let rc = unsafe {
            let mut rc = 1;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut rc);
            rc
        };

        if rc == 0 {
            let len = unsafe {
                let mut len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                len
            };
            
            let err = {
                let mut buf = Vec::with_capacity(len as usize + 1);
                buf.extend([b'\0'].iter().cycle().take(len as usize));
                unsafe {
                    let cstr = CString::from_vec_unchecked(buf);
                    gl::GetProgramInfoLog(id, len, ptr::null_mut(), cstr.as_ptr() as *mut GLchar);
                    cstr
                }
            };
            Err(Box::new(ProgramError::CreationFailed(err.to_string_lossy().into_owned())))
        } else {
            Ok(Self { id, vert_shader, frag_shader })
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    // `name` must be null terminated.
    pub fn uniform_mat4_location(&self, name: &str) -> Option<GLint> {
        let loc = unsafe {
            gl::GetUniformLocation(self.id, name.as_ptr() as *const GLchar)
        };
        if loc == -1 {
            None
        } else {
            Some(loc)
        }
    }

    pub fn uniform_mat4_set(&self, loc: GLint, mat: &Mat4) {
        unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.as_slice().as_ptr() as *const GLfloat);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DetachShader(self.id, self.vert_shader.id());
            gl::DetachShader(self.id, self.frag_shader.id());
            gl::DeleteProgram(self.id);
        }
    }
}
