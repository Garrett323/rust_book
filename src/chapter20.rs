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
use rust_book::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;

fn web_server_main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read from stream
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // send response only if we get the right request
    let get = b"GET / HTTP/1.1.\r\n"; // define correct request to 'root' of our
                                      // webserver
    let sleep = b"GET /sleep HTTP/1.1.\r\n"; // simulate slow request
    let (status_line, filname) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n{}", "resources/hello.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n{}", "resources/hello.html")
    } else {
        // other requests
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "resources/404.html")
    };
    let contents = std::fs::read_to_string(filname).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
