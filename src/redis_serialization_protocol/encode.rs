use crate::redis_serialization_protocol::RESPValue;

pub fn encode_resp_value(value:RESPValue) -> String {
    match value {
        RESPValue::SimpleString(s) => format!("+{}\r\n", s),
        RESPValue::Error(s) => format!("-{}\r\n", s),
        RESPValue::Integer(i) => format!(":{}\r\n", i),
        RESPValue::BulkString(arr) => format!("${}\r\n{}\r\n", arr.len(), String::from_utf8(arr).unwrap()),
        RESPValue::Array(vec) => {
            let mut result = String::new();
            result.push('*');
            result.push_str(&vec.len().to_string());
            result.push('\r');
            result.push('\n');
            for val in vec {
                result.push_str(&encode_resp_value(val));
            }
            result
        }
        RESPValue::NULL => "_\r\n".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_string() {
        assert_eq!(encode_resp_value(RESPValue::SimpleString("OK".to_string())), "+OK\r\n");
    }

    #[test]
    fn test_encode_error() {
        assert_eq!(encode_resp_value(RESPValue::Error("ERR wrong".to_string())), "-ERR wrong\r\n");
    }

    #[test]
    fn test_encode_integer() {
        assert_eq!(encode_resp_value(RESPValue::Integer(42)), ":42\r\n");
    }

    #[test]
    fn test_encode_bulk_string() {
        assert_eq!(encode_resp_value(RESPValue::BulkString("hello world".as_bytes().to_vec())), "$11\r\nhello world\r\n");
    }

    #[test]
    fn test_encode_array() {
        let array = vec![RESPValue::SimpleString("OK".to_string()), RESPValue::Error("ERR wrong".to_string()), RESPValue::Integer(42)];
        assert_eq!(encode_resp_value(RESPValue::Array(array)), "*3\r\n+OK\r\n-ERR wrong\r\n:42\r\n");
    }

    #[test]
    fn test_encode_null() {
        assert_eq!(encode_resp_value(RESPValue::NULL), "_\r\n");
    }
}