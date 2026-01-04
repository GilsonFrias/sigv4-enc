//use crate::encoder::{encode};
mod encoder;

fn main() {
    println!("Hello, world!");
    let result = encoder::encode(encoder::HttpMethod::GET, "foo@faa.com/users?userId=1");
    println!("{}", result); 
}
