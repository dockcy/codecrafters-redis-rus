use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Uncomment this block to pass the first stage use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Start handling a new connection");
                thread::spawn(|| {
                handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buf = [0u8; 128];
        match stream.read(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                stream.write_all(b"+PONG\r\n").expect("write response error");
            }
            Err(e) => { eprintln!("{e}");break; }
        }
    }
}
