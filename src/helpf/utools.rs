use std::io::Read;
pub use std::{process, fs::File, error::Error, ffi::OsString, env, io::{self, BufRead}, path::Path};

pub fn get_arg_n(n : usize) -> std::result::Result<OsString, Box<dyn Error>> {
    //matching on 
    match env::args_os().nth(n) {
        Some(file_path) => Ok(file_path),
        None=>Err(From::from("err")),
    }
}


pub fn load_file(file_n : usize) -> Result<String, Box<dyn Error>>{

    let path = get_arg_n(file_n).expect("Couldn't find file in provided path.");
    let mut file = File::open(path).expect("Can't open provided file.");
    let mut buf =String::new();
    file.read_to_string(&mut buf).expect("Couldn't load file contents into buffer.");

    Ok(buf)

}