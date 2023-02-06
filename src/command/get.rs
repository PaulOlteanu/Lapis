use lapis_resp::RespType;

use crate::db::Command;

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn new(args: &[&str]) -> Option<Self> {
        if args.len() != 1 {
            return None;
        }

        let res = Self {
            key: args[0].to_string(),
        };

        Some(res)
    }
}

impl Command for Get {
    fn execute(&self, db: &crate::db::Db) -> Result<Option<RespType>, ()> {
        if let Ok(map) = db.map.lock() {
            map.get(&self.key);
        }

        Ok(None)
    }
}
