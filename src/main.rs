//I don't want to do merge:: for every module we've created. 
use merge::{*, helpf::shader_utils::*, helpf::types::*};

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
   
    let mut obj = Object::<Vec3<f32>>::default();
    obj.data = vec![[-0.5, -0.5, 0.0].into(), [0.5, -0.5, 0.0].into(), [0.0, 0.5, 0.0].into()];

    let mut render_group: RenderDecal = RenderDecal::default();
    render_group.gen_vo();
    render_group.bind_vo();
    render_group.set_vertex_attributes();
    render_group.add_object(&mut obj);
    render_group.bind_buffer();
 
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
            //glDrawArrays(GL_TRIANGLES, 0, 3);
            render_group.draw_objects();
        }
        win.swap_window();
    }

}



