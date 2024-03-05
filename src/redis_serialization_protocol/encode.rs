use crate::redis_serialization_protocol::*;

pub fn encode_resp_value(value:&RESPValue) -> String {
    match value {
        RESPValue::SimpleString(s) => format!("+{}\r\n", s),
        RESPValue::Error(s) => format!("-{}\r\n", s.clone()),
        RESPValue::Integer(i) => format!(":{}\r\n", i),
        RESPValue::BulkString(BulkEnumerator::Value(arr)) => format!("${}\r\n{}\r\n", arr.len(), String::from_utf8(arr.clone()).unwrap()),
        RESPValue::BulkString(BulkEnumerator::Null) => "$-1\r\n".to_string(),
        RESPValue::BulkString(BulkEnumerator::Empty) => "$0\r\n\r\n".to_string(),
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
        let value = RESPValue::SimpleString("OK".to_string());
        let expected = "+OK\r\n";
        assert_eq!(encode_resp_value(&value), expected);
    }

    #[test]
    fn test_encode_error() {
        let value = RESPValue::Error("ERR wrong number of arguments for 'set' command".to_string());
        let expected = "-ERR wrong number of arguments for 'set' command\r\n";
        assert_eq!(encode_resp_value(&value), expected);
    }

    #[test]
    fn test_encode_integer() {
        let value = RESPValue::Integer(42);
        let expected = ":42\r\n";
        assert_eq!(encode_resp_value(&value), expected);
    }

    #[test]
    fn test_encode_bulk_string() {
        let value = RESPValue::BulkString(BulkEnumerator::Value(b"hello world".to_vec()));
        let expected = "$11\r\nhello world\r\n";
        assert_eq!(encode_resp_value(&value), expected);
    }

    #[test]
    fn test_encode_null_bulk_string() {
        let value = RESPValue::BulkString(BulkEnumerator::Null);
        let expected = "$-1\r\n";
        assert_eq!(encode_resp_value(&value), expected);
    }

    #[test]
    fn test_encode_empty_bulk_string() {
        let value = RESPValue::BulkString(BulkEnumerator::Empty);
        let expected = "$0\r\n\r\n";
        assert_eq!(encode_resp_value(&value), expected);
    }

    #[test]
    fn test_encode_array() {
        let value = RESPValue::Array(vec![
            RESPValue::SimpleString("OK".to_string()),
            RESPValue::Error("ERR wrong number of arguments for 'set' command".to_string()),
            RESPValue::Integer(42),
            RESPValue::BulkString(BulkEnumerator::Value(b"hello world".to_vec())),
            RESPValue::BulkString(BulkEnumerator::Null),
            RESPValue::BulkString(BulkEnumerator::Empty),
        ]);
        let expected = "*6\r\n+OK\r\n-ERR wrong number of arguments for 'set' command\r\n:42\r\n$11\r\nhello world\r\n$-1\r\n$0\r\n\r\n";
        assert_eq!(encode_resp_value(&value), expected);
    }

    #[test]
    fn test_encode_null() {
        let value = RESPValue::NULL;
        let expected = "_\r\n";
        assert_eq!(encode_resp_value(&value), expected);
    }
}