use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Listen for connections on 7878
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Failed to bind");
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Determine which file to serve
    let (status_line, contents) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", include_str!("../assets/hello.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND", include_str!("../assets/error.html"))
    };

    // Read the contents of the file
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to write response");
}