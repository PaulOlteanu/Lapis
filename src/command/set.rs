use lapis_resp::RespType;

use crate::db::Command;

#[derive(Debug)]
pub enum Conditional {
    Nx,
    Xx,
}

#[derive(Debug)]
pub enum Expiry {
    Ex,
    Px,
    Exat,
}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: String,

    conditional: Option<Conditional>,
    get: bool,
    expiry: Option<Expiry>,
    keep_ttl: bool,
}

impl Set {
    pub fn new(args: &[&str]) -> Option<Self> {
        if args.len() < 2 {
            return None;
        }

        let mut res = Self {
            key: args[0].to_string(),
            value: args[1].to_string(),

            conditional: None,
            get: false,
            expiry: None,
            keep_ttl: false,
        };

        for arg in &args[2..] {
            match arg.to_ascii_lowercase().as_str() {
                "nx" => res.conditional = Some(Conditional::Nx),
                "xx" => res.conditional = Some(Conditional::Xx),

                "get" => res.get = true,

                "ex" => res.expiry = Some(Expiry::Ex),
                "px" => res.expiry = Some(Expiry::Px),
                "exat" => res.expiry = Some(Expiry::Exat),

                "keepttl" => res.keep_ttl = true,

                _ => return None,
            }
        }

        Some(res)
    }
}

impl Command for Set {
    fn execute(&self, db: &crate::db::Db) -> Result<Option<RespType>, ()> {
        if let Ok(mut map) = db.map.lock() {
            map.insert(self.key.clone(), self.value.clone());
        }

        Ok(None)
    }
}
