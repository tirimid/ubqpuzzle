use gl::types::{GLenum, GLuint, GLchar};
use std::ptr;
use std::ffi::{CString, CStr};
use std::path::Path;
use std::fs;
use std::fmt;
use std::error::Error;

pub struct Shader {
    id: GLuint,
}

#[derive(Debug)]
pub enum ShaderError {
    CreationFailed(String),
}

impl Error for ShaderError {}
impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Shader {
    pub fn from_file<P: AsRef<Path>>(path: P, shader_type: GLenum) -> Result<Self, ShaderError> {
        let src = CString::new(fs::read_to_string(path).unwrap()).unwrap();
        Self::from_source(&src, shader_type)
    }
    
    pub fn from_source(src: &CStr, shader_type: GLenum) -> Result<Self, ShaderError> {
        let id = unsafe {
            gl::CreateShader(shader_type)
        };
        
        unsafe {
            gl::ShaderSource(id, 1, &src.as_ptr(), ptr::null());
            gl::CompileShader(id);
        }

        let rc = unsafe {
            let mut rc = 1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut rc);
            rc
        };
        
        if rc == 0 {
            let len = unsafe {
                let mut len = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                len
            };
            
            let err = {
                let mut buf = Vec::with_capacity(len as usize + 1);
                buf.extend([b'\0'].iter().cycle().take(len as usize));
                unsafe {
                    let cstr = CString::from_vec_unchecked(buf);
                    gl::GetShaderInfoLog(id, len, ptr::null_mut(), cstr.as_ptr() as *mut GLchar);
                    cstr
                }
            };
            Err(ShaderError::CreationFailed(err.to_string_lossy().into_owned()))
        } else {
            Ok(Self { id })
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
