use lapis_resp::RespType;

use crate::db::Command;

#[derive(Debug)]
pub struct Ping {
    response: Option<String>,
}

impl Ping {
    pub fn new(args: &[&str]) -> Option<Self> {
        if args.len() > 1 {
            return None;
        }

        let mut res = Self { response: None };
        if args.len() == 1 {
            res.response = Some(args[0].to_string());
        }

        Some(res)
    }
}

impl Command for Ping {
    fn execute(&self, _db: &crate::db::Db) -> Result<Option<RespType>, ()> {
        if let Some(res) = &self.response {
            Ok(Some(RespType::BulkString(Some(res.clone()))))
        } else {
            Ok(Some(RespType::SimpleString("PONG".to_string())))
        }
    }
}
