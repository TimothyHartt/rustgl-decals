use ogl33::*;
use iunorm::*;

//Buffer Data Types
pub trait BufferType : Sized {
    const SIZE : isize = std::mem::size_of::<Self>() as isize;
}

#[derive(Clone, Default)]
pub struct Vec3<T> {
    x : T,
    y : T,
    z : T
}

impl<T : Copy> From<[T;3]> for Vec3<T> {
    fn from(value: [T;3]) -> Self {
        Vec3 { x: value[0], y: value[1], z: value[2] }
    }
}

impl From<[f32; 3]> for Vec3<Unorm8> {
    fn from(value: [f32; 3]) -> Self {
        Vec3 { x : Unorm8::from_f32(value[0]), y : Unorm8::from_f32(value[1]), z : Unorm8::from_f32(value[2])}
    }
}

impl<T> BufferType for Vec3<T>{}

//Render batch Groups
pub trait RenderGroup {
    fn gen_vo(&mut self);
    fn bind_vo(&self);
    fn draw_objects(&self);
    fn bind_buffer(&self);
}

#[derive( Default)]
pub struct RenderDecal {
    pub data : Vec<Vec3<f32>>,
    pub vao : u32,
    pub vbo : u32,
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
        unsafe{
            glDrawArrays(GL_TRIANGLES, 0, self.data.len() as i32);
        }
    }

    fn bind_buffer(&self) {
        unsafe {
            glBufferData(GL_ARRAY_BUFFER, Vec3::<f32>::SIZE * (self.data.len() as isize),
            self.data.as_ptr().cast(), GL_STATIC_DRAW);
        }
    }
    
}

impl RenderDecal {
    pub fn set_vertex_attributes(&self) {
        unsafe {
            glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE,
                0, 0 as *const _);
            glEnableVertexAttribArray(0);
        }
    }

    pub fn add_object(&mut self, obj : &mut Object<Vec3<f32>>){
        self.data.append(&mut obj.data);
    }
}

#[derive( Default)]
//Entities that will be rendered. This will need to be carved
pub struct Object<T:Clone>{
    pub data : Vec<T>
}
