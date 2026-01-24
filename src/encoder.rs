use dotenv::dotenv;
use std::env;
use url::{Url, ParseError};

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
            
            //2. Construct sorted canonical queries, fallback to an empty string if no 
            //queries are available
            let canonical_queries = if let Some(q) = queries {
                //2.1 Sort query strings alphabetically and concatenate them back
                let mut queries: Vec<&str> = q.split('&').collect();
                queries.sort();
                queries.join("?")
            }else {
                "".to_string()
            };
            
            //3 Replace reserved characters
            //TODO: wrap reserved chars replacement routine
            //in a centralized Regex-based function and replace in place
            let canonical_queries = canonical_queries.replace("+", "%20");
            let canonical_queries = canonical_queries.replace("*", "%2A");
            
            //4 Concat canonical_uri(path) and cononical_queries with line return \n char
            let canonical_request = format!("{:?}\n{:?}", path, canonical_queries);
            return canonical_request
        }
        Err(_) => {
            //TODO: handle specific ParseError
            println!("Something went wrong parsing the url");
            return url.to_string()   
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
