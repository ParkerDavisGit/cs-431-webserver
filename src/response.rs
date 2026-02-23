use std::{collections::HashMap, fmt::Display, fs::File, hash::Hash, io::{BufReader, prelude::*}, os::windows::fs::MetadataExt, str::FromStr, time::SystemTime};

use chrono::Date;

use crate::{http_date::HttpDate, request::Request, response};

const STATIC_DATA_LOCATION: &str = "static/";
const SERVER_VERSIONS: &str = "CS531Server/a1 Rust/1.85.0";
const HTTP_VERSION: &str = "1.1";

type MethodFunction = fn(&Request, Response) -> Response;

pub struct Response {
    request_method: String,
    http_response_code: i32,
    http_response_status: String,
    date: HttpDate,
    content_length: usize,
    connection_status: String,
    allow: Option<Vec<String>>,
    body: Vec<u8>,
    file_mime_type: String,
    last_modified: HttpDate
}

impl Response {
pub fn new_from_request(request: Request) -> Response {
    // Instantiate with date (It is the easiest)
    let response: Response = Response {
        request_method: Default::default(), 
        http_response_code: Default::default(), 
        http_response_status: Default::default(),
        date: HttpDate::get_current(),
        content_length: Default::default(),
        connection_status: Default::default(),
        allow: None,
        body: Vec::new(),
        file_mime_type: Default::default(),
        last_modified: HttpDate::get_current()
    };

    // Then, check if HTTP version is correct
    // Only accepting HTTP 1.1
    if request.get_http_version().as_str() != "1.1" {
        return Self::set_response_status(response, 505);
    }

    if !request.get_header_is_correct() {
        return Self::set_response_status(response, 400);
    }

    if request.get_host() == None {
        return Self::set_response_status(response, 400)
    }

    let response = Self::set_request_method(&request, response);

    // Set connection status
    // For now, the connection may only be closed. Or left out, implied closed.
    let response= Self::set_connection_status(&request, response);


    let response = match Self::get_response_methods().get(&request.get_http_method()) {
        Some(method) => {
            method(&request, response)
        }
        None => {
            Self::set_response_status(response, 400)
        }
    };

    response
}

fn get_request(request: &Request, mut response: Response) -> Response {
    match File::open(format!("static/{}", request.get_request_path())) {
        Ok(mut file) => {
            file.read_to_end(&mut response.body).unwrap();

            response.content_length = response.body.len();
            //response.last_modified = HttpDate::from_system_time(file.metadata().unwrap().modified().unwrap());
            //println!("{}", file.metadata().unwrap().modified().unwrap());

            Self::set_response_status(
                Self::set_mime_type(response, request.get_request_path()),
                200
            )
        }
        Err(_) => {
            println!("{}", request.get_request_path());
            Self::set_response_status(response, 404)
        }
    }
}

fn head_request(request: &Request, mut response: Response) -> Response {
    match File::open(format!("static/{}", request.get_request_path())) {
        Ok(mut file) => {
            response.content_length = file.metadata().unwrap().file_size() as usize;
            //response.last_modified = HttpDate::from_system_time(file.metadata().unwrap().modified().unwrap());

            Self::set_response_status(
                Self::set_mime_type(response, request.get_request_path()),
                200
            )
        }
        Err(_) => {
            println!("{}", request.get_request_path());
            Self::set_response_status(response, 404)
        }
    }
}

fn options_request(request: &Request, mut response: Response) -> Response {
    response.allow = Some(Self::get_response_methods().keys().map(|str| str.to_string()).collect());
    Self::set_response_status(response, 200)
}

fn trace_request(request: &Request, mut response: Response) -> Response {
    response.body = format!("{}", request).as_bytes().to_vec();
    response
}

fn unimplemented_request(request: &Request, mut response: Response) -> Response {
    Self::set_response_status(response, 501)
}

// Rewrite this please
// If the provided connection isn't `close` or `keep-alive`, do I treat as close or 
//   Throw error?
fn set_connection_status(request: &Request, mut response: Response) -> Response {
    match request.get_connection_status() {
        Some(status) => {
            match status.as_str() {
                "close" => {
                    response.connection_status = "close".to_string();
                }

                // This is where `keep-alive` will go, however that is not
                //   implemented yet.

                _ => {
                    response.connection_status = "close".to_string();
                }
            }
        }
        None => {
            response.connection_status = "close".to_string();
        }
    };

    response
}

fn set_response_status(mut response: Response, status_code: i32) -> Response {
    match status_code {
        200 => { 
            response.http_response_code = 200;
            response.http_response_status = "OK".to_string();
        },
        400 => { 
            response.http_response_code = 400;
            response.http_response_status = "Bad Request".to_string();
        },
        403 => { 
            response.http_response_code = 403;
            response.http_response_status = "Forbidden".to_string();
        },
        404 => { 
            response.http_response_code = 404;
            response.http_response_status = "Not Found".to_string();
        },
        500 => { 
            response.http_response_code = 500;
            response.http_response_status = "Internal Server Error".to_string();
        },
        501 => { 
            response.http_response_code = 501;
            response.http_response_status = "Not Implemented".to_string();
        },
        505 => { 
            response.http_response_code = 505;
            response.http_response_status = "HTTP Version Not Supported".to_string();
        },
        // Just incase
        _ => { 
            response.http_response_code = 500;
            response.http_response_status = "Internal Server Error".to_string();
        }
    }
    response
}

fn set_mime_type(mut response: Response, file_path: String) -> Response {
    let file_extension = match file_path.split(".").last() {
        Some(extentsion) => extentsion,
        None => "thisll become an octet stream",
    };

    let mime_type = match file_extension.to_lowercase().as_str() {
        "txt" => { "text/plain" }
        "htm" => { "text/html" }
        "html" => { "text/html" }
        "xml" => { "text/xml" }
        "png" => { "image/png" }
        "jpeg" => { "image/jpeg" }
        "jpg" => { "image/jpeg" }
        "gif" => { "image/gif" }
        "pdf" => { "application/pdf" }
        "ppt" => { "application/vnd.ms-powerpoint" }
        "doc" => { "application/vnd.ms-word" }
        "http" => { "message/http" }
        _ => { "application/octet-stream" }
    }.to_string();

    response.file_mime_type = mime_type;

    response
}

fn set_request_method(request: &Request, mut response: Response) -> Response {
    response.request_method = request.get_http_method();
    response
}


fn get_response_methods() -> HashMap<String, MethodFunction> {
    let mut accepted_http_methods: HashMap<String, MethodFunction> = HashMap::new();

    accepted_http_methods.insert("GET".to_string(), Response::get_request);
    accepted_http_methods.insert("HEAD".to_string(), Response::head_request);
    accepted_http_methods.insert("OPTIONS".to_string(), Response::options_request);
    accepted_http_methods.insert("TRACE".to_string(), Response::trace_request);
    accepted_http_methods.insert("POST".to_string(), Response::unimplemented_request);
    accepted_http_methods.insert("PUT".to_string(), Response::unimplemented_request);
    accepted_http_methods.insert("DELETE".to_string(), Response::unimplemented_request);
    accepted_http_methods.insert("CONNECT".to_string(), Response::unimplemented_request);
    accepted_http_methods.insert("PATCH".to_string(), Response::unimplemented_request);

    accepted_http_methods
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
            "Date: {}\r\nServer: {}\r\nConnection: {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n", 
            self.date,
            SERVER_VERSIONS,
            self.connection_status,
            self.content_length,
            self.file_mime_type
        ));

        match self.allow.clone() {
            Some(options) => {
                let _ = write!(f, "Allow: {}\r\n", options.join(", "));
            },
            None => {}
        };

        let _ = write!(f, "\r\n");

        Ok(())
    }
}

// Extra methods for Display
impl Response {
    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }
}