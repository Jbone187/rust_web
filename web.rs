use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connected")
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("index.html").unwrap();
    let response = format!("{status_line}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
}
