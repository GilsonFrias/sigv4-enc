use dotenv::dotenv;
use std::env;

pub enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
}


pub fn encode(HTTPMethod: HttpMethod) -> String {
    dotenv().ok();
    let access_key = env::var("ACCESS_KEY").expect("ACCESS KEY is not configured");
    return "foo".to_string()
}
