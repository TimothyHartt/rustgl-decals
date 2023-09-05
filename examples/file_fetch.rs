#[path = "./helpf/utools.rs"]
mod helpf;


fn main() {

    println!("{:#?}", helpf::load_file(1).unwrap());

}