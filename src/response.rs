use std::{fmt::Display, str::FromStr};

use crate::{http_date::HttpDate, request::Request};

const STATIC_DATA_LOCATION: &str = "static/";
const HTTP_VERSION: &str = "1.1";

pub struct Response {
    http_response_code: i32,
    http_response_status: String,
    date: HttpDate
}

impl Response {
pub fn new_from_request(request: Request) -> Response {
    // Instantiate with date (It is the easiest)
    let mut response: Response = Response {
        http_response_code: Default::default(), 
        http_response_status: Default::default(),
        date: HttpDate::get_current()
    };

    // Then, check if HTTP version is correct
    // Only accepting HTTP 1.1
    if request.get_http_version().as_str() != "1.1" {
        let response_status = Self::get_response_status(505);
        response.http_response_code = response_status.0;
        response.http_response_status = response_status.1;
        return response;
    }


    match request.get_http_method().as_str() {
        "GET" => {
            println!("Apples")
        }
        _ => {
            println!("Hmm")
        }
    }



    response
}

fn get_request(mut request: Request, mut response: Response) -> Response {
    response
}

fn get_response_status(status_code: i32) -> (i32, String) {
    match status_code {
        200 => { (200, "OK".to_string()) },
        505 => { (505, "HTTP Version Not Supported".to_string()) },
        // Just incase
        _ => { (500, "Internal Server Error".to_string()) }
    }
}
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Top bit
        let _ = write!(f, "{}", format!(
            "HTTP/{} {} {}\r\n", 
            HTTP_VERSION,
            self.http_response_code,
            self.http_response_status
        ));

        // Rest of the headers
        let _ = write!(f, "{}", format!(
            "Date: {}\r\n\r\n", 
            self.date
        ));

        Ok(())
    }
}
