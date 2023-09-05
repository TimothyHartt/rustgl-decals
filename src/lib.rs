//This is dumb, maybe not dumb, but stupid dumb. 
/*
We use lib.rs to define our file structure. Both folders can be modules and so can .rs files.
We need the pub keyword to expose the modules to other files that peek inside this.  
*/


pub use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
};

pub use beryllium::*;
pub use ogl33::*;


pub mod helpf {
    pub mod utools;
    pub mod shader_utils;
}