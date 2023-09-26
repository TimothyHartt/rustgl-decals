//I don't want to do merge:: for every module we've created. 
use merge::{*, helpf::shader_utils::*};
use std::mem;
//Pos datatye for shader input

/*
Opengl programs are not statically linked, they are linked at runtime, using drivers and features
based on the user's system. There is uncertainty, your calls may fail, this is unsafe code in rust
*/

/*
    Lets try and draw something more interesting. I want to put thing into a seperate glsl file, so lets figure that out.
    I want to remake my grass project but with more complex shapes for the blades of grass and how they animate according to some external "wind".   
    It's be great to have a way to load .obj or other 3d model file types. 


    First, glsl files. 

*/

/* 
Stage Two:
    "World Objects"
    Contains a World Matrix
        - Includes Translation, Rotation, and Scale Mats
    Possibly contains what texture(s) it needs that has been loaded on the gpu.
    That should be it. It really shouldn't have a functon maybe besides some constuctor.

    To do future:
    World, View, Projection Matrix
    For now we (probably don't need any of these for our decal test.)

*/



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
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);

        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
    }
}

fn set_vertex_attributes(){
    unsafe{
        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE,
            size_of::<Vec3>().try_into().unwrap(), 0 as *const _);
        glEnableVertexAttribArray(0);

    }
}

//Will take an inpute, with different function definitions for some situations 
fn bind_buffer_data(){


    let verts: Vec<Vec3> = 
        vec![[-0.5, -0.5, 0.0].into(), [0.5, -0.5, 0.0].into(), [0.0, 0.5, 0.0].into()];
    
    unsafe{
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
        /*

            //We need to bind an array object and at least 1 buffer
            //We need to setup the vertex attribute poiter, a template telling the GPU how the data being sent is organized.
            //Need a vertex attributte pointer for each attribute, and need to enable it after each
            We need the verticies that we will draw. I normally hate coming up with these manually, but whatevs 
            We need to bind them to a buffer.
            At some point we need to setup our graphics pipeline with a Vertex and Fragment shader.
            Then clear the screan, then send our data to the gpu with the glDraw function
            And finally we swap buffers to draw to the string, coolio 


            First, lets create the shaders. 
        */
        //Creat Vertex Array & Buffer Objects
        //These can really be in any order, just need to bind vertex obejects before binding or modify bound objects

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



/*
Of course functional programing has patterns. These exist to carry out common tasks while being safe, easy to do
and re-usable. They get you thinking in a certain way. I think my time in OOAD make me really look down on the idea.
It makes a lot of sense though. So, how about this. Lets start from zero and see if this increases the workload I can do.

struct Color(i32, i32, i32);
    A struct with no way to map a name to the values it holds. Can use self.0, self.1, ect.
    So, when we say we use a struct to wrap a value what we're wrapping is the self.0, or whatever.
    The fuctions and potential other values are just there to modify this wrapped value. 


    

 */