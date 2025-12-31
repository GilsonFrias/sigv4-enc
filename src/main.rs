//use crate::encoder::{encode};
mod encoder;

fn main() {
    println!("Hello, world!");
    let result = encoder::encode(encoder::HttpMethod::GET, "faa");
    println!("{}", result); 
}
