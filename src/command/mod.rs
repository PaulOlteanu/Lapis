use lapis_resp::RespType;

pub mod get;
pub mod set;

use get::Get;
use set::Set;

use crate::db::Command;

// TODO: Make this better
pub fn from_resp(resp: &RespType) -> Result<Box<dyn Command>, ()> {
    if let RespType::Array(Some(arr)) = resp {
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
                    return Ok(Box::new(set));
                }
            }

            "get" => {
                if let Some(get) = Get::new(&unwrapped[1..]) {
                    return Ok(Box::new(get));
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
