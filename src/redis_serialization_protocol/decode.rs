use std::error::Error;
use std::{i32, io, usize};

use crate::redis_serialization_protocol::{BulkEnumerator, RESPValue};

pub fn get_resp_value(input: &mut &[u8]) -> Result<RESPValue, Box<dyn Error>> {
    let input_type = input.get(0).ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid RESP input"))?.clone();
    *input = &input[1..];
    match input_type {
        b'+' => {
            let line = read_line(input);
            return Ok(RESPValue::SimpleString(String::from_utf8_lossy(line).to_string()));
        }
        b'-' => {
            let line = read_line(input);
            Ok(RESPValue::Error(String::from_utf8_lossy(line).to_string()))
        }
        b':' => {
            let line = read_line(input);
            Ok(RESPValue::Integer(i32::from_str_radix(&String::from_utf8_lossy(line), 10)?))
        }
        b'$' => {
            let length_line = read_line(input);
            let length = i32::from_str_radix(&String::from_utf8_lossy(length_line), 10).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid RESP input"))?;
            match length {
                0 => {
                    Ok(RESPValue::BulkString(BulkEnumerator::Empty))
                }
                ..=-1 => {
                    Ok(RESPValue::BulkString(BulkEnumerator::Null))
                }
                len => {
                    let len = len as usize;
                    let data = read_line(input);
                    if data.len()  < len  {
                        return Err(Box::try_from(io::Error::new(io::ErrorKind::InvalidData, "Invalid RESP input")).unwrap());
                    }
                    Ok(RESPValue::BulkString(BulkEnumerator::Value(data[..len].to_vec())))
                }
            }
        }
        b'*' => {
            let length_line = read_line(input);
            let length = usize::from_str_radix(&String::from_utf8_lossy(length_line), 10).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid RESP input"))?;
            if length > 0 {
                let mut array = vec![];
                for _ in 0..length {
                    // See the magic of stream function `read_line()`!
                    array.push(get_resp_value(input).unwrap());
                }
                Ok(RESPValue::Array(array))
            } else {
                Ok(RESPValue::NULL)
            }
        }
        _ => {
            Ok(RESPValue::NULL)
        }
    }
}

// As "\r\n" is the separator in Redis Protocol character stream
// a universal slicing function is necessary
// This function use the design pattern called "Stream"
// which means the "input" can be consumed and move to next position


fn read_line<'a>(input: &mut &'a [u8]) -> &'a [u8] {
    // if let Some(index) = input.windows(2).position(|w| w == [b'\\', b'r']) {
    //     let line = &input[..index];
    //     *input = &input[index + 4..];
    if let Some(index) = input.windows(2).position(|w| w == [b'\r', b'\n']) {
        let line = &input[..index];
        *input = &input[index + 2..];
        line
    } else {
        let line = *input;
        //Don't continue while all chars has been read;
        *input = &[];
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_line() {
        let mut input: &[u8] = b"+OK\r\n";
        assert_eq!(read_line(&mut input), b"+OK");
        assert_eq!(input, b"");
    }

    #[test]
    fn test_get_resp_value_simple_string() {
        let mut input: &[u8] = b"+OK\r\n";
        assert_eq!(get_resp_value(&mut input).unwrap(), RESPValue::SimpleString("OK".to_string()));
        assert_eq!(input, b"");
    }

    #[test]
    fn test_get_resp_value_bulk_string() {
        let mut input: &[u8] = b"$4\r\nWiki\r\n";
        assert_eq!(get_resp_value(&mut input).unwrap(), RESPValue::BulkString(BulkEnumerator::Value(b"Wiki".to_vec())));
        assert_eq!(input, b"");
    }

    #[test]
    fn test_get_resp_value_array() {
        let mut input: &[u8] = b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        assert_eq!(get_resp_value(&mut input).unwrap(), RESPValue::Array(vec![RESPValue::BulkString(BulkEnumerator::Value(b"foo".to_vec())), RESPValue::BulkString(BulkEnumerator::Value(b"bar".to_vec()))]));
        assert_eq!(input, b"");
    }

    #[test]
    fn test_get_resp_value_error() {
        let mut input: &[u8] = b"-Error message\r\n";
        assert_eq!(get_resp_value(&mut input).unwrap(), RESPValue::Error("Error message".to_string()));
        assert_eq!(input, b"");
    }

    #[test]
    fn test_get_resp_value_integer() {
        let mut input: &[u8] = b":1000\r\n";
        assert_eq!(get_resp_value(&mut input).unwrap(), RESPValue::Integer(1000));
        assert_eq!(input, b"");
    }

    #[test]
    fn test_get_resp_value_null() {
        let mut input: &[u8] = b"$-1\r\n";
        assert_eq!(get_resp_value(&mut input).unwrap(), RESPValue::BulkString(BulkEnumerator::Null));
        assert_eq!(input, b"");
    }

}

