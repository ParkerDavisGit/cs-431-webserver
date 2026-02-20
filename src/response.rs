use std::fmt::{Display};

const HTTP_VERSION: f32 = 1.1f32;

pub struct Response {
    http_response_code: i32,
    http_response_status: String,
}

impl Response {
    pub fn new() -> Response {
        Response { 
            http_response_code: 200i32, 
            http_response_status: "OK".to_string()
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", format!(
            "HTTP/{} {} {}\r\n\r\n", 
            HTTP_VERSION,
            self.http_response_code,
            self.http_response_status
        ));
        Ok(())
    }
}
