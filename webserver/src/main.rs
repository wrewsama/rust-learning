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
    let req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("req: {req:#?}");

    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap();
    let len = content.len();

    let resp = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");

    stream.write_all(resp.as_bytes()).unwrap();
}
