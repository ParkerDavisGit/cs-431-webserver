use std::fmt::{Display, write};

const HTTP_VERSION: f32 = 1.1f32;

pub struct Response {
    http_response_code: i32,
    http_response_status: String,
}

impl Response {
    pub fn new() -> ResponseBuilder<Empty> {
        ResponseBuilder::new()
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




/// Builder Stuff
// States
pub struct Empty;
pub struct WithCode;
pub struct WithStatus;


pub struct ResponseBuilder<STATE> {
    http_response_code: i32,
    http_response_status: String,

    state: STATE
}


impl ResponseBuilder<Empty> {
    pub fn new () -> ResponseBuilder<Empty>{
        ResponseBuilder { 
            http_response_code: 0, 
            http_response_status: "".to_string(), 
            state: Empty
        }
    }
}


impl ResponseBuilder<Empty> {
    pub fn with_code(self, new_code: i32) -> ResponseBuilder<WithCode> {
        ResponseBuilder { 
            http_response_code: new_code, 
            http_response_status: self.http_response_status, 
            state: WithCode 
        }
    }
}

impl ResponseBuilder<WithCode> {
    pub fn with_status(self, new_status: String) -> ResponseBuilder<WithStatus> {
        ResponseBuilder { 
            http_response_code: self.http_response_code, 
            http_response_status: new_status, 
            state: WithStatus 
        }
    }
}


impl ResponseBuilder<WithStatus> {
    pub fn build(self) -> Response {
        Response { 
            http_response_code: self.http_response_code, 
            http_response_status: self.http_response_status
        }
    }
}
