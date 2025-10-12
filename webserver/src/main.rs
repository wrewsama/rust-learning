use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream_res in listener.incoming() {
        let stream = stream_res.unwrap();
        println!("connection established");

        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) { // mutable ref to stream bc we need to write to it
    let buf_reader = BufReader::new(&stream);
    let req = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();


    let (status, filename) = if req == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let len = content.len();

    let resp = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");

    stream.write_all(resp.as_bytes()).unwrap();
}
