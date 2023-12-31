pub use std::mem;
use ogl33::*;
//use std::ffi::CString;
// /use core::mem::{size_of, size_of_val};

use super::utools;



pub trait ShaderProgram {
    fn compile_shader(&mut self) -> &mut Self;
    fn link_program(&mut self) -> &mut Self;
    fn use_program(&self);
    fn cach_uniforms(&mut self) -> &mut Self;
}

#[derive(Default)]
pub struct SimpleShader {
    vertex_shader : u32,
    fragment_shader : u32,
    shader_program : u32,
    pub uniform_list : Vec<(String, i32)>
}

impl ShaderProgram for SimpleShader {
    
    fn compile_shader(&mut self) -> &mut Self {
        unsafe {
            self.shader_program = glCreateProgram(); 

        //Setup Vertex Shader
            self.vertex_shader = glCreateShader(GL_VERTEX_SHADER);
            assert_ne!(self.vertex_shader, 0);
    
            let vert_shader = utools::load_file(1).unwrap();
            glShaderSource(self.vertex_shader, 1, &(vert_shader.as_str().as_bytes().as_ptr().cast()), 
                &(vert_shader.len().try_into().unwrap()));
    
            glCompileShader(self.vertex_shader);
    
            //Check for Errors in Vertex Shader
            check_shader_err(self.vertex_shader);
            glAttachShader(self.shader_program, self.vertex_shader);


        //Setup Fragment Shader
            self.fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
            assert_ne!(self.fragment_shader, 0);

            let frag_shader= utools::load_file(2).unwrap();
            glShaderSource(self.fragment_shader, 1, &(frag_shader.as_str().as_bytes().as_ptr().cast()), 
                &(frag_shader.len().try_into().unwrap()));

            glCompileShader(self.fragment_shader);
            //Check for Frag Errors
            check_shader_err(self.fragment_shader);
            glAttachShader(self.shader_program, self.fragment_shader);



        }

        self
    }

    fn link_program(&mut self) -> &mut Self {
        unsafe {
            glLinkProgram(self.shader_program);
            //let mut max_length : GLint = 0;
            //glGetProgramiv(self.shader_program, GL_ACTIVE_UNIFORMS, &mut max_length);
            //println!("{:#?}", max_length);
          
        }
        self
    }

    fn use_program(&self){
        unsafe {
            glUseProgram(self.shader_program);
        }

    }

    fn cach_uniforms(&mut self) -> &mut Self {
        unsafe{
            //We'd repeat these steps for each uniform.
            let name : *const i8 = b"color\0".as_ptr() as *const i8;

            let loc = glGetUniformLocation(self.shader_program, name);
            self.uniform_list.push(("color".to_string(), loc));
        }

        println!("{:#?}", self.uniform_list);
        self
    }
    
}

impl Drop for SimpleShader {
    fn drop(&mut self) {
        unsafe {
            glDeleteShader(self.vertex_shader);
            glDeleteShader(self.fragment_shader);
            glDeleteProgram(self.shader_program)
        }
    }
}



fn check_shader_err(shader : u32){
    let mut success = 0;
    unsafe{
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
              shader,
              1024,
              &mut log_len,
              v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }
}