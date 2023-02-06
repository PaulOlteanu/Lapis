use std::{collections::HashMap, sync::Mutex};

use enum_dispatch::enum_dispatch;

use crate::{command::set::Set, command::CommandType};
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

    pub fn run_command(&self, cmd: &CommandType) -> Option<RespType> {
        cmd.execute(self);
        None
    }
}

#[enum_dispatch(CommandType)]
pub trait Command {
    fn execute(&self, db: &Db) -> Result<(), ()>;
}
