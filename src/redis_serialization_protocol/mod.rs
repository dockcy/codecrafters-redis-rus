pub mod encode;
pub mod decode;

#[derive(Debug, PartialEq)]
pub enum RESPValue{
    NULL,
    SimpleString(String),
    Error(String),
    Integer(i32),
    BulkString(BulkEnumerator),
    Array(Vec<RESPValue>)
}

#[derive(Debug,PartialEq)]
pub enum BulkEnumerator {
    Value(Vec<u8>),
    Empty,
    Null,
}