use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Uncomment this block to pass the first stage use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read the request
    let mut req_buf = [0u8; 1024];
    let _bytes_num = stream.read(&mut req_buf).unwrap();
    let req_string = String::from_utf8_lossy(&req_buf);
    println!("request content :{req_string}");
    let response = concat!(
        "+PONG\r\n"
    ).as_bytes();
    let _ = stream.write(response).unwrap();
    stream.flush().expect("flush error");
    println!("finish the response!");
}
