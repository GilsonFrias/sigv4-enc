use dotenv::dotenv;
use std::env;
use url::{Url, ParseError};

pub enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
}

fn urlEncode(url: &str) -> String {
    //1. Obtain absolute path from url
    //2. URLEncode.encode the string params according to AWS docu
    //Create Canonical request template
    //let parsed = Url::parse(url)?;
    let parsed = Url::parse("https://foo@foo.com/?myQuery=#yes!");
    match parsed {
        Ok(path) => {
            println!("The base path is: {:?}", path.username());
        }
        Err(_) => {
            println!("Something went wrong parsing the url"); 
        }
    }
    return "str".to_string()
}


pub fn encode(httpmethod: HttpMethod, uri: &str) -> String {
    dotenv().ok();
    let access_key = env::var("ACCESS_KEY").expect("ACCESS KEY is not configured");
    let test_uri = "https://foo@faa.com/?YourQueryString=True";
    urlEncode(test_uri);
    return format!("{} {}", access_key, uri)
}
