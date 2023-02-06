use enum_dispatch::enum_dispatch;
use lapis_resp::RespType;

pub mod set;

use set::Set;

#[derive(Debug)]
#[enum_dispatch]
pub enum CommandType {
    Set,
}

fn get_arg(arg: &RespType) -> Option<&str> {
    if let RespType::BulkString(Some(s)) = arg {
        Some(s.as_str())
    } else {
        None
    }
}

impl TryFrom<RespType> for CommandType {
    type Error = ();

    fn try_from(value: RespType) -> Result<Self, Self::Error> {
        if let RespType::Array(Some(arr)) = value {
            if arr.is_empty() {
                return Err(());
            }

            let mut unwrapped = Vec::new();

            for val in arr.iter() {
                if let RespType::BulkString(Some(val)) = val {
                    unwrapped.push(val.as_str());
                } else {
                    return Err(());
                }
            }

            if unwrapped.is_empty() {
                return Err(());
            }

            match unwrapped[0].to_ascii_lowercase().as_str() {
                "set" => {
                    if let Some(set) = Set::new(&unwrapped[1..]) {
                        return Ok(CommandType::Set(set));
                    }
                }
                _ => {
                    return Err(());
                }
            }

            return Err(());
        } else {
            Err(())
        }
    }
}

// impl Command {}

// impl Executable for Command {
//     fn execute(&self, db: &mut crate::db::Db) {
//     }
// }

// pub enum Command {
//     Set(Type),
//     Get,
// }

// impl Command {
//     pub fn parse(cmd: Type) -> Option<Self> {
//         if let Type::Array(cmd) = cmd {

//             match cmd[0] {
//                 "set" =>
//             }
//         } else {
//             None
//         }
//     }
// }
