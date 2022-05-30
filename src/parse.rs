#[derive(Debug)]
pub enum Command {
    GET,
    SET,
    UPDATE,
    DELETE,
    None,
}

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
        let command = command.trim().to_uppercase();

        if &command[..3] == "GET" && command.chars().nth(3).unwrap() == ' ' {
            res.command = Command::GET;
            res.key = command[4..].to_string();
        }else if &command[..3] == "SET" && command.chars().nth(3).unwrap() == ' ' {
            let kv: Vec<&str> = command[4..].split(' ').collect();
            if kv.len() != 2 {
                return res;
            }

            res.command = Command::SET;
            res.key = kv[0].to_string();
            res.value = kv[1].to_string();

        }else if &command[..6] == "UPDATE" && command.chars().nth(6).unwrap() == ' ' {
            let kv: Vec<&str> = command[7..].split(' ').collect();
            if kv.len() != 2 {
                return res;
            }

            res.command = Command::UPDATE;
            res.key = kv[0].to_string();
            res.value = kv[1].to_string();

        }else if &command[..6] == "DELETE" && command.chars().nth(6).unwrap() == ' ' {
            res.command = Command::DELETE;
            res.key = command[7..].to_string();
        }
        res
    }
}