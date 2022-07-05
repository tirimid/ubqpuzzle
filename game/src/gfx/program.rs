use gl::types::{GLchar, GLuint};
use crate::gfx::shader::Shader;
use std::path::Path;
use std::ffi::CString;
use std::ptr;

pub struct Program {
    id: GLuint,
    vert_shader: Shader,
    frag_shader: Shader,
}

impl Program {
    pub fn new<P: AsRef<Path>>(vert_shader_path: P, frag_shader_path: P) -> Result<Self, String> {
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
            Err(err.to_string_lossy().into_owned())
        } else {
            Ok(Self { id, vert_shader, frag_shader })
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}