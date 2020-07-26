use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::{read};
use colored::Colorize;

/// Simple single thread file server.
pub fn server() {
    println!("{}: Started server at http://localhost:8080", "[SERVER]".yellow());

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    };
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request: String = String::from_utf8_lossy(&buffer).parse().unwrap();

    let mut request_split = request.split(" ");

    let method = request_split.next().unwrap_or("GET");

    let path = request_split.next().unwrap_or("/");

    println!("{}: {} {}", "[SERVER]".yellow(), method, path);

    let byte_content = read("./nopl.html").expect("./nopl.html should exist.");

    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
        byte_content.len()
    ).as_bytes().to_vec();

    response.extend(byte_content);

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}