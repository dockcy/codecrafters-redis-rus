#[derive(Debug,PartialEq)]
pub enum Command {
    ECHO,
    PING,
    SET,
    GET,
}

pub fn redis_command_parser(command: String) -> Option<Command> {
    match command.to_lowercase().as_str() {
        "ping" => Some(Command::PING),
        "set" => Some(Command::SET),
        "get" => Some(Command::GET),
        "echo" => Some(Command::ECHO),
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
        assert_eq!(redis_command_parser("PING".to_string()), Some(Command::PING));
        assert_eq!(redis_command_parser("SET".to_string()), Some(Command::SET));
        assert_eq!(redis_command_parser("GET".to_string()), Some(Command::GET));
        assert_eq!(redis_command_parser("ECHO".to_string()), Some(Command::ECHO));
        assert_eq!(redis_command_parser("unknown".to_string()), None);
    }
}