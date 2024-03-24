use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use redis_command_parser::*;
use redis_serialization_protocol::*;

use crate::redis_serialization_protocol::decode::get_resp_value;
use crate::redis_serialization_protocol::encode::encode_resp_value;
use crate::schedualer::ValueProperties;

mod redis_serialization_protocol;
mod redis_command_parser;
mod schedualer;

fn main() {
    println!("Sever starting...");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let map:Arc<Mutex<HashMap<String,ValueProperties>>> = Arc::new(Mutex::new(HashMap::new()));
    for stream in listener.incoming() {
        let map = map.clone();
        match stream {
            Ok(stream) => {
                println!("Start handling a new connection");
                thread::spawn(move || {
                    handle_connection(stream,map);
                });
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, map: Arc<Mutex<HashMap<String, ValueProperties>>>) {
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
                        match arr.get(0) {
                            Some(RESPValue::BulkString(bulk_str)) => {
                                match bulk_str {
                                    BulkEnumerator::Value(cmd) => {
                                        match redis_command_parser(String::from_utf8(cmd.to_vec()).unwrap()) {
                                            Some(Command::PING) => {
                                                let encode_string = encode_resp_value(&RESPValue::BulkString(BulkEnumerator::Value(b"PONG".to_vec())));
                                                stream.write_all(encode_string.as_bytes()).expect("write response error");
                                            }
                                            Some(Command::ECHO) => {
                                                if let Some(v) = arr.get(1) {
                                                    let encode_string = encode_resp_value(v);
                                                    stream.write_all(encode_string.as_bytes()).expect("write response error");
                                                } else {
                                                    stream.write_all(b"-ERR invalid arguments\r\n").expect("write response error");
                                                }
                                            }
                                            Some(Command::SET) => {
                                                match arr.as_slice() {
                                                    [_,RESPValue::BulkString(BulkEnumerator::Value(key)),RESPValue::BulkString(BulkEnumerator::Value(value))] => {
                                                        let value = ValueProperties::new(String::from_utf8(value.to_vec()).unwrap(), Instant::now(), None);
                                                        map.lock().unwrap().entry(String::from_utf8(key.to_vec()).unwrap()).or_insert(value);
                                                        stream.write_all(b"+OK\r\n").expect("write response error");
                                                    }
                                                    [_,RESPValue::BulkString(BulkEnumerator::Value(key)),RESPValue::BulkString(BulkEnumerator::Value(value)),_,RESPValue::BulkString(BulkEnumerator::Value(expire_time))] => {
                                                        let value = ValueProperties::new(String::from_utf8(value.to_vec()).unwrap(), Instant::now(), u128::from_str_radix(String::from_utf8(expire_time.to_vec()).unwrap().as_str(), 10).ok());
                                                        map.lock().unwrap().entry(String::from_utf8(key.to_vec()).unwrap()).or_insert(value);
                                                        stream.write_all(b"+OK\r\n").expect("write response error");
                                                    }
                                                    _ => {
                                                        stream.write_all(b"-ERR invalid arguments\r\n").expect("write response error");
                                                    }
                                                }
                                            }
                                            Some(Command::GET) => {
                                                if let Some(RESPValue::BulkString(BulkEnumerator::Value(key))) = arr.get(1) {
                                                    if let Some(value) = map.lock().unwrap().get(&String::from_utf8(key.to_vec()).unwrap()) {
                                                        if value.is_expired() {
                                                            let encode_string = encode_resp_value(&RESPValue::BulkString(BulkEnumerator::Null));
                                                            stream.write_all(encode_string.as_bytes()).expect("write response error");
                                                        } else {
                                                            let encode_string = encode_resp_value(&RESPValue::BulkString(BulkEnumerator::Value(value.value.as_bytes().to_vec())));
                                                            stream.write_all(encode_string.as_bytes()).expect("write response error");
                                                        }
                                                    } else {
                                                        let encode_string = encode_resp_value(&RESPValue::BulkString(BulkEnumerator::Null));
                                                        stream.write_all(encode_string.as_bytes()).expect("write response error");
                                                    }
                                                } else {
                                                    stream.write_all(b"-ERR invalid arguments\r\n").expect("write response error");
                                                }
                                            }
                                            _ => {
                                                // other commands
                                                unimplemented!()
                                            }
                                        }
                                    }
                                    _ => {
                                        stream.write_all(b"-ERR invalid command\r\n").expect("write response error");
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

