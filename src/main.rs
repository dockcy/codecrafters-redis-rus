use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use redis_command_parser::*;
use redis_serialization_protocol::*;

use crate::redis_serialization_protocol::decode::get_resp_value;

mod redis_serialization_protocol;
mod redis_command_parser;

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
                match get_resp_value(&mut &buf[..n]) {
                    Ok(RESPValue::Array(arr)) => {
                        match arr.get(0)  {
                            Some(RESPValue::BulkString(cmd)) => {
                                match redis_command_parser(String::from_utf8(cmd.to_vec()).unwrap()) {
                                    Some(Command::PING) => {
                                        stream.write_all(b"PONG\r\n").expect("write response error");
                                    }
                                    Some(Command::ECHO) => {
                                        if let Some(RESPValue::BulkString(v)) = arr.get(1) {
                                            stream.write_all(v).expect("write response error");
                                        } else {
                                            stream.write_all(b"-ERR invalid arguments\r\n").expect("write response error");
                                        }
                                    }
                                    _ => {
                                        unimplemented!()
                                    }
                                }
                            }
                            _ => {
                                stream.write_all(b"-ERR invalid command\r\n").expect("write response error");
                            }
                        }
                    }
                    Ok(_) => {
                        unimplemented!()
                    }
                    Err(e) => {
                        stream.write_all(b"-ERR invalid command\r\n").expect("write response error");
                        eprintln!("{e}");
                        break;
                    }
                }

            }
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
    }
}

