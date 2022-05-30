#[derive(Debug)]
pub enum Command {
    GET,
    SET,
    UPDATE,
    DELETE,
    None,
}

/*
    DB Syntax parser.
    Four commands, get set update delete, are supported.
    The command is not case sensitive, but key and value are.
 */

#[derive(Debug)]
pub struct Parse {
    pub command: Command,
    pub key: String,
    pub value: String,
}

impl Parse {
    pub fn parse(bytes: Vec<u8>) -> Self {
        let mut res = Parse {
            command: Command::None,
            key: String::new(),
            value: String::new(),
        };
        let command = match std::str::from_utf8(bytes.as_slice()) {
            Ok(T) => T,
            Err(_) => return res,
        };
        let command = command.trim();

        if &command[..3].to_uppercase() == "GET" && command.chars().nth(3).unwrap() == ' ' {
            res.command = Command::GET;
            res.key = command[4..].to_string();
        }else if &command[..3].to_uppercase() == "SET" && command.chars().nth(3).unwrap() == ' ' {
            let kv: Vec<&str> = command[4..].split(' ').collect();
            if kv.len() != 2 {
                return res;
            }

            res.command = Command::SET;
            res.key = kv[0].to_string();
            res.value = kv[1].to_string();

        }else if &command[..6].to_uppercase() == "UPDATE" && command.chars().nth(6).unwrap() == ' ' {
            let kv: Vec<&str> = command[7..].split(' ').collect();
            if kv.len() != 2 {
                return res;
            }

            res.command = Command::UPDATE;
            res.key = kv[0].to_string();
            res.value = kv[1].to_string();

        }else if &command[..6].to_uppercase() == "DELETE" && command.chars().nth(6).unwrap() == ' ' {
            res.command = Command::DELETE;
            res.key = command[7..].to_string();
        }
        res
    }
}