use std::{collections::HashMap, fmt::Display, io::{BufReader, prelude::*}, net::TcpStream};
use std::result::Result;

pub struct Request {
    request_method: String,
    request_path: String,
    http_version: String,
    host: Option<String>,
    connection: Option<String>,
}

pub enum RequestError {
    HostNotFound
}

// Standard HTTP Request Structure: ORDER IS ARBITRARY after the first line
// <Request Method> <Path> HTTP/<HTTP Version>
// Host: <host uri>
// Connection: <status:> (This is only 'close' for assignment 1)
impl Request {
    pub fn handle_request(stream: &TcpStream) -> Result<Request, i32> {
        let buf_reader = BufReader::new(stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        
        let mut http_iterator = http_request.into_iter();

        let mut request_header = http_iterator.next().unwrap().split(" ").map(|twine| twine.to_string()).collect::<Vec<String>>().into_iter();
        let mut request_headers: HashMap<&str, String> = HashMap::new();

        for line in http_iterator {
            let mut split_line = line.split(": ");
            match split_line.next().unwrap() {
                "Host" => { request_headers.insert("Host", split_line.next().unwrap().to_string()); },
                "Connection" => { request_headers.insert("Connection", split_line.next().unwrap().to_string()); },
                _ => {}
            }
        }

        //println!("{}", request_headers.get("Connection").unwrap());

        Ok(Request {
            request_method: request_header.next().unwrap(),
            request_path: request_header.next().unwrap(),
            http_version: request_header.next().unwrap().split("/").last().unwrap().to_string(),
            host: request_headers.get("Host").cloned(),
            connection: request_headers.get("Connection").cloned()
        })
    }
}

// Getters
impl Request {
    pub fn get_http_method(&self) -> String { self.request_method.clone() }
    pub fn get_http_version(&self) -> String { self.http_version.clone() }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", format!(
            "{} {} HTTP/{}\n",
            self.request_method,
            self.request_path,
            self.http_version
        ));

        let _ = match &self.host {
            Some(host) => {
                write!(f, "Host: {}\n", host)
            }
            None => { Ok(()) }
        };
        let _ = match &self.connection {
            Some(host) => {
                write!(f, "Connection: {}\n", host)
            }
            None => { Ok(()) }
        };

        Ok(())
    }
}