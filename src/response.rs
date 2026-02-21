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
    let response: Response = Response {
        http_response_code: 200i32, 
        http_response_status: "OK".to_string(),
        date: HttpDate::get_current()
    };

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

fn get_request(request: Request, mut response: Response) -> Response {
    response
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
