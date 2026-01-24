use dotenv::dotenv;
use std::env;
use url::{Url, ParseError};
use percent_encoding::{percent_encode, AsciiSet};

const ENCODE_SET: &AsciiSet = &percent_encoding::NON_ALPHANUMERIC.remove(b'-').remove(b'.').remove(b'_').remove(b'~');
const PATH_ENCODE_SET: &AsciiSet = &ENCODE_SET.remove(b'/');

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
}

fn url_encode(url: &str) -> String {
    /*
    URLEncode.encode the path and query str params according to AWS sigV4 encoding specs
    */

    //1. URL parse the input url string
    let parsedurl = Url::parse(url);
    match parsedurl {
        Ok(url) => {
            //1. Obtain paths and queries from url
            let path = url.path();
            let queries = url.query();
            
            //2. Encode the path
            let encoded_path = percent_encode(path.as_bytes(), PATH_ENCODE_SET).to_string();
            
            //3. Construct sorted canonical queries, fallback to an empty string if no 
            //queries are available
            let canonical_queries = if let Some(q) = queries {
                // Parse query pairs
                let mut pairs: Vec<(String, String)> = q.split('&').filter_map(|pair| {
                    let mut parts = pair.splitn(2, '=');
                    let key = parts.next()?.to_string();
                    let value = parts.next().unwrap_or("").to_string();
                    Some((key, value))
                }).collect();
                
                // Sort by key
                pairs.sort_by(|a, b| a.0.cmp(&b.0));
                
                // Encode and format
                let encoded_pairs: Vec<String> = pairs.into_iter().map(|(k, v)| {
                    format!("{}={}", percent_encode(k.as_bytes(), ENCODE_SET), percent_encode(v.as_bytes(), ENCODE_SET))
                }).collect();
                
                encoded_pairs.join("&")
            } else {
                "".to_string()
            };
            
            //4 Concat canonical_uri(path) and canonical_queries with line return \n char
            let canonical_request = format!("{}\n{}", encoded_path, canonical_queries);
            canonical_request
        }
        Err(_) => {
            //TODO: handle specific ParseError
            println!("Something went wrong parsing the url");
            url.to_string()   
        }
    }
}


pub fn encode(httpmethod: HttpMethod, uri: &str) -> String {
    dotenv().ok();
    //Obtain ACCESS_KEY from env variables
    let access_key = env::var("ACCESS_KEY").expect("ACCESS KEY is not configured");

    let test_uri = "https://foo@faa.com/test/scripts/users?User=Pedro P.~&YourQueryString1=*True+True*&zvalue=&YourQueryString2=_My/-Value/~26_&Zvalue=_";
    //let test_uri = "https://foo@faa.com?User=Pedro P.~&YourQueryString1=*True+True*&zvalue=&YourQueryString2=_My/-Value/~26_&Zvalue=_";
    println!("Test url: {:?}", test_uri);

    //encode uri to form the base for the canonical_request
    let canonical_request = url_encode(test_uri);
    println!("The canonical_url is: {:?}", canonical_request);

    //concat the HttpMethod to the beginning of the canonical_request string
    let canonical_request = format!("{:?}\n{:?}", httpmethod, canonical_request);
    println!("The canonical_url is: {:?}", canonical_request);
    return canonical_request
}
