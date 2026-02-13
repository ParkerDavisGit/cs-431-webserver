use std::{fmt::Display, io::{BufReader, prelude::*}, net::TcpStream};


pub struct Request {
    
}

impl Request {
    pub fn handle_request(stream: &TcpStream) -> Request {
        let buf_reader = BufReader::new(stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        
        for line in http_request {
            println!("{}", line);
        }

        Request {
            
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", format!(
            "",
        ));
        Ok(())
    }
}