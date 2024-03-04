use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::redis_serialization_protocol::decode::get_resp_value;

mod redis_serialization_protocol;
mod redis_command_parser;
use redis_serialization_protocol::*;
use redis_command_parser::*;

fn main() {
    println!("Sever starting...");

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
        let mut buf = [0u8; 1024];
        match stream.read(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                println!("Get req");
                if let Ok(RESPValue::Array(arr)) = get_resp_value(&mut &buf[..n]) {
                    if arr.len() != 2 {
                        println!("Not valid command");
                        continue;
                    }
                    println!("Get command: {:?}",&arr);
                    if let Some(RESPValue::BulkString(cmd)) = arr.get(0) {
                        if let Ok(cmd_str) = String::from_utf8(cmd.to_vec()) {
                            if let Some(Command::ECHO) = redis_command_parser(cmd_str) {
                                if let Some(RESPValue::BulkString(v)) = arr.get(1) {
                                    stream.write_all(v).expect("write response error");
                                }
                            }
                        }
                    }
                } else {
                    println!("Not valid command: {:?}",String::from_utf8_lossy(&buf[..n]));
                    stream.write_all(b"-ERR invalid command\r\n").expect("write response error");
                }
            }
            Err(e) => { eprintln!("{e}");break; }
        }
    }
}

