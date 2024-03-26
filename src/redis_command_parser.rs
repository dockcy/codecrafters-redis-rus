#[derive(Debug,PartialEq)]
pub enum RedisCommand {
    INFO,
    ECHO,
    PING,
    SET,
    GET,
}

impl TryFrom<String> for RedisCommand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "info" => Ok(RedisCommand::INFO),
            "ping" => Ok(RedisCommand::PING),
            "set" => Ok(RedisCommand::SET),
            "get" => Ok(RedisCommand::GET),
            "echo" => Ok(RedisCommand::ECHO),
            _ => Err(format!("Invalid command: {}", value)),
        }
    }
}

pub fn redis_command_parser(command: String) -> Option<RedisCommand> {
    match command.to_lowercase().as_str() {
        "ping" => Some(RedisCommand::PING),
        "set" => Some(RedisCommand::SET),
        "get" => Some(RedisCommand::GET),
        "echo" => Some(RedisCommand::ECHO),
        "info" => Some(RedisCommand::INFO),
        _ => {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_command_parser() {
        assert_eq!(redis_command_parser("PING".to_string()), Some(RedisCommand::PING));
        assert_eq!(redis_command_parser("SET".to_string()), Some(RedisCommand::SET));
        assert_eq!(redis_command_parser("GET".to_string()), Some(RedisCommand::GET));
        assert_eq!(redis_command_parser("ECHO".to_string()), Some(RedisCommand::ECHO));
        assert_eq!(redis_command_parser("unknown".to_string()), None);
    }
}