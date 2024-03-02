pub mod encode;
pub mod decode;

#[derive(Debug, PartialEq)]
pub enum RESPValue{
    NULL,
    SimpleString(String),
    Error(String),
    Integer(i32),
    BulkString(Vec<u8>),
    Array(Vec<RESPValue>)
}