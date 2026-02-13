use std::{
    fs::File, io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}
};

use cs431_web_server::request::{self, Request};
use cs431_web_server::response::Response;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let mut current_stream: TcpStream;

    for stream in listener.incoming() {
        //current_stream = stream.unwrap();

        // By default for this assignment, all connections are closed (thrown out of scope)
        // To keep it connected, you can just keep the stream in scope.
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request: Request = Request::handle_request(&stream);

    let response: Response = Response::new()
        .with_code(200)
        .with_status("OK".to_string())
        .build();

    let mut html_string: String = "".to_string();
    let html_file = File::open("html/index.html").unwrap().read_to_string(&mut html_string);

    let response = format!("{}{}", response, &html_string);

    stream.write_all(response.as_bytes()).unwrap();
}