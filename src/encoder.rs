use dotenv::dotenv;
use std::env;

pub enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
}


pub fn encode(httpmethod: HttpMethod, uri: &str) -> String {
    dotenv().ok();
    let access_key = env::var("ACCESS_KEY").expect("ACCESS KEY is not configured");
    return format!("{} {}", access_key, uri)
}
