use dotenv::dotenv;
use std::env;
use url::{Url, ParseError};

pub enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
}

fn url_encode(url: &str) -> String {
    //2. URLEncode.encode the string params according to AWS docu
    //Create Canonical request template
    //let parsed = Url::parse(url)?;
    //let parsed = Url::parse("https://foo@foo.com/?myQuery=#yes!");
    let parsedurl = Url::parse(url);
    match parsedurl {
        Ok(url) => {
            //1. Obtain paths and queries from url
            let path = url.path();
            let queries = url.query();
            let path_and_queries = if let Some(q) = queries {
                println!("Queries succesfully unpacked: {:?}", q);
                //3. Sort query strings alphabetically
                let mut queries: Vec<&str> = q.split('&').collect();
                println!("Resultant split: {:?}", queries);
                queries.sort();
                println!("Resultant sorted queries: {:?}", queries);
                format!("{}?{}", path, q)
            }else {
                path.to_string()
            };
            //2. Replace spaces with '+'
            let encoded = path_and_queries.replace("%20", "%22"); 
            //let encoded = format!("{}{}", path, queries); 
            println!("The complete url is: {:?}", url);
            println!("The base path is: {:?}", path);
            println!("The set of query strings: {:?}", queries);
            println!("The resultant encoded string: {:?}", encoded);
        }
        Err(_) => {
            println!("Something went wrong parsing the url"); 
        }
    }
    return "str".to_string()
}


pub fn encode(httpmethod: HttpMethod, uri: &str) -> String {
    dotenv().ok();
    //Obtain ACCESS_KEY form env variables
    let access_key = env::var("ACCESS_KEY").expect("ACCESS KEY is not configured");
    let test_uri = "https://foo@faa.com/test/scripts/users?User=Pedro P.&YourQueryString1=*True*&YourQueryString2=_My-Value-26_&Zvalue=_";
    url_encode(test_uri);
    /*
    let parsedurl = Url::parse(test_uri);
    match parsedurl {
        Ok(url) => {
            //1. Obtain absolute path from url
            let path = url.path();   
            println!("The base path is: {:?}", path);
        }
        Err(_) => {
            println!("Something went wrong parsing the url"); 
        }
    }
    */
    return format!("{} {}", access_key, uri)
}
