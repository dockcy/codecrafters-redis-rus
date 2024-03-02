use std::error::Error;
use std::io;

use crate::redis_serialization_protocol::RESPValue;

pub fn get_resp_value(input: &mut &[u8]) -> Result<RESPValue,Box<dyn Error>> {
    let input_type = input.get(0).ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid RESP input"))?.clone();
    *input = &input[1..];
    match input_type {
        b'+' => {
            let line = read_line(input);
            return  Ok(RESPValue::SimpleString(String::from_utf8_lossy(line).to_string()))
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
            let length = usize::from_str_radix(&String::from_utf8_lossy(length_line), 10).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid RESP input"))?;
            if length > 0 {
                let data = read_line(input);
                if data.len() < length {
                    return Err(Box::try_from(io::Error::new(io::ErrorKind::InvalidData, "Invalid RESP input")).unwrap());
                }
                Ok(RESPValue::BulkString(data[..length].to_vec()))
            } else { Ok(RESPValue::BulkString(vec![])) }
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
                Ok(RESPValue::Array(vec![]))
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
    if let Some(index) = input.iter().position(|&b| b == b'\r') {
        let line = &(input[..index]);
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
    fn test_get_respvalue_simple_string() {
        let mut input = b"+OK\r\n".as_ref();
        let expected = RESPValue::SimpleString("OK".to_string());
        assert_eq!(get_resp_value(&mut input).unwrap(), expected);
    }

    #[test]
    fn test_get_respvalue_error() {
        let mut input = b"-Error message\r\n".as_ref();
        let expected = RESPValue::Error("Error message".to_string());
        assert_eq!(get_resp_value(&mut input).unwrap(), expected);
    }

    #[test]
    fn test_get_respvalue_integer() {
        let mut input = b":1000\r\n".as_ref();
        let expected = RESPValue::Integer(1000);
        assert_eq!(get_resp_value(&mut input).unwrap(), expected);
    }

    #[test]
    fn test_get_respvalue_bulk_string() {
        let mut input = b"$6\r\nfoobar\r\n".as_ref();
        let expected = RESPValue::BulkString(b"foobar".to_vec());
        assert_eq!(get_resp_value(&mut input).unwrap(), expected);
    }

    #[test]
    fn test_get_respvalue_array() {
        let mut input = b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".as_ref();
        let expected = RESPValue::Array(vec![
            RESPValue::BulkString(b"foo".to_vec()),
            RESPValue::BulkString(b"bar".to_vec()),
        ]);
        assert_eq!(get_resp_value(&mut input).unwrap(), expected);
    }

    #[test]
    fn test_get_respvalue_nested_array() {
        let mut input = b"*2\r\n*1\r\n$3\r\nfoo\r\n*2\r\n$3\r\nbar\r\n$3\r\nbaz\r\n".as_ref();
        let expected = RESPValue::Array(vec![
            RESPValue::Array(vec![
                RESPValue::BulkString(b"foo".to_vec())
            ]),
            RESPValue::Array(vec![
                RESPValue::BulkString(b"bar".to_vec()),
                RESPValue::BulkString(b"baz".to_vec()),
            ])
        ]);
        assert_eq!(get_resp_value(&mut input).unwrap(), expected);
    }


}

