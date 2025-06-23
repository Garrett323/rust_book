pub fn run() {
    println!("Chapter 20: Multithreaded Webserver");
    web_server_main();
}

/*
* PLAN:
* -> learn about TCP and HTTP
* -> Listen for TCP connections on a socket
* -> Parse small number of HTTP requests
* -> Create proper HTTP response
* -> Improve the throughput of pour server with a thread pool
*/
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn web_server_main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
