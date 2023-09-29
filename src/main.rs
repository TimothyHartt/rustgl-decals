//I don't want to do merge:: for every module we've created. 
use merge::{*, helpf::shader_utils::*, helpf::types::*};

//Pos datatye for shader input

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

fn set_vertex_attributes1(){
    unsafe{

        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE,
            0, 0 as *const _);
        glEnableVertexAttribArray(0);

       

    }
}

fn bind_buffer_data(){


    let mut verts: Vec<Vec3::<f32>> = 
        vec![[-0.5, -0.5, 0.0].into(), [0.5, -0.5, 0.0].into(), [0.0, 0.5, 0.0].into()];
    
    unsafe{
        //glBindBuffer(GL_ARRAY_BUFFER, )
        glBufferData(GL_ARRAY_BUFFER, Vec3::<f32>::SIZE * (verts.len() as isize),
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


fn init_gl(win : &GlWindow) {
    unsafe{
        load_gl_with(|fn_name| win.get_proc_address(fn_name));

       
        bind_vo();
        set_vertex_attributes1();
        bind_buffer_data();

        //These Should be moved out of here
        
        let mut shader = helpf::shader_utils::SimpleShader::default();
        shader.compile_shader().link_program().cach_uniforms().use_program();

        
        glUniform3f(shader.uniform_list[0].1, 0.0f32, 0.0f32, 30.0f32);

        glEnable(GL_DEPTH_TEST);
    }
}

fn main() {
    let sdl = init_sdl();

    //Setup Window
    let win = sdl
    .create_gl_window("Hellow Rustgl",
    WindowPosition::Centered,
        600,
        600,
        WindowFlags::Shown).expect("Couldn't create a window and Context");
    sdl.gl_set_attribute(SdlGlAttr::DepthSize, 32).unwrap();
    //Init gl, load&compile shaders, bind vertex objects, set uniform
    init_gl(&win);



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



