//use crate::encoder::{encode};
mod encoder;

fn main() {
    println!("Hello, world!");
    encoder::encode(encoder::HttpMethod::GET);
}
