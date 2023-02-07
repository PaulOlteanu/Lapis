use std::{collections::HashMap, fmt::Debug, sync::Mutex};

use lapis_resp::RespType;

pub struct Db {
    pub map: Mutex<HashMap<String, String>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn run_command(&self, cmd: &dyn Command) -> Result<Option<RespType>, ()> {
        cmd.execute(self)
    }
}

pub trait Command: Debug + Send {
    fn execute(&self, db: &Db) -> Result<Option<RespType>, ()>;
}
