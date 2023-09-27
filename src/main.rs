//I don't want to do merge:: for every module we've created. 
use merge::{*, helpf::shader_utils::*};
use std::mem;
//Pos datatye for shader input


trait RenderGroup {
    fn gen_vo(&mut self);
    fn bind_vo(&self);
    fn draw_objects(&self);
}

struct RenderDecal {
    data : Vec<Vertex_Data>,
    vao : u32,
    vbo : u32,
}

impl RenderGroup for RenderDecal {
    fn gen_vo(&mut self) {
        unsafe{
            glGenVertexArrays(1, &mut self.vao);
            glGenBuffers(1, &mut self.vbo);
        }
    }
    fn bind_vo(&self) {
        unsafe{
            glBindVertexArray(self.vao);
            glBindBuffer(GL_ARRAY_BUFFER, self.vbo);
        }
        
    }
    
    fn draw_objects(&self) {
        let t_size = mem::size_of::<Vertex_Data>() as isize;
        unsafe{
            glBufferData(GL_ARRAY_BUFFER, t_size * (self.data.len() as isize),
            self.data.as_ptr().cast(), GL_STATIC_DRAW);
            glDrawArrays(GL_TRIANGLES, 0, self.data.len() as i32);
        }
    }
}

impl RenderDecal {
    fn set_vertex_attributes() {
        unsafe {
            glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE,
                size_of::<Vertex_Data>().try_into().unwrap(), 0 as *const _);
            glEnableVertexAttribArray(0);

            glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE,
                size_of::<Vertex_Data>().try_into().unwrap(), size_of::<Vec3>() as *const _);
            glEnableVertexAttribArray(1);
        }
    }

    fn add_object(&mut self, obj : &mut Object<Vertex_Data>){
        self.data.append(&mut obj.data);
    }
}

fn bind_vo(){
    unsafe{
        let mut vao = 0;
        let mut vbo = 0;
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);

        
        glGenBuffers(1, &mut vbo);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        let mut instance_vbo = 0;
        glGenBuffers(1, &mut instance_vbo);
        glBindBuffer(GL_ARRAY_BUFFER, instance_vbo);

    }
}

fn set_vertex_attributes(){
    unsafe{
        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE,
            size_of::<Vec3>().try_into().unwrap(), 0 as *const _);
        glEnableVertexAttribArray(0);

        glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE,
            size_of::<Vec3>().try_into().unwrap(), 0 as *const _);
        glEnableVertexAttribArray(0);
        glVertexAttribDivisor(1, 1);

    }
}

fn bind_buffer_data(){


    let verts: Vec<Vec3> = 
        vec![[-0.5, -0.5, 0.0].into(), [0.5, -0.5, 0.0].into(), [0.0, 0.5, 0.0].into()];

    let pos_a : Vec<Vec3> = vec![[0.25, 0.25, 0.0].into(), [0.0, 0.0, 0.0].into()];
    
    unsafe{
        //glBindBuffer(GL_ARRAY_BUFFER, )
        glBufferData(GL_ARRAY_BUFFER, VEC3_SIZE * (verts.len() as isize),
        verts.as_ptr().cast(), GL_STATIC_DRAW);
    }
}

fn init_sdl() -> SDL {
    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");

    //Set Opengl version 
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 4).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 6).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
    sdl
        .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
        .unwrap();
    }


    sdl
}

fn main() {
    let sdl = init_sdl();

    let win = sdl
    .create_gl_window("Hellow Rustgl",
    WindowPosition::Centered,
        600,
        600,
        WindowFlags::Shown).expect("Couldn't create a window and Context");

    unsafe{
        sdl.gl_set_attribute(SdlGlAttr::DepthSize, 32).unwrap();
        load_gl_with(|fn_name| win.get_proc_address(fn_name));

        let mut shader = helpf::shader_utils::SimpleShader::default();
        shader.compile_shader().link_program().cach_uniforms().use_program();
        glUniform3f(shader.uniform_list[0].1, 0.0f32, 0.0f32, 30.0f32);

        bind_vo();
        set_vertex_attributes();
        bind_buffer_data();
        
        glEnable(GL_DEPTH_TEST);
    }

    //Loops until it hits a break. Compiler doesn't panic if there is no break. Gotta be careful with these.
    'main_loop: loop {

        while let  Some(e) = sdl.poll_events().and_then(Result::ok) {
            match e {
                Event::Quit(_) => break 'main_loop,
                _ => (),
            }
        }
        unsafe{
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
        win.swap_window();

    }

}



