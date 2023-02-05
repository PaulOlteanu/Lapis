use std::{collections::HashMap, sync::Mutex};

use enum_dispatch::enum_dispatch;

use crate::{command::set::Set, command::CommandType, resp::Type};

pub struct Db {
    pub map: Mutex<HashMap<String, String>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn run_command(&self, cmd: &CommandType) -> Option<Type> {
        cmd.execute(self);
        None
    }
}

#[enum_dispatch(CommandType)]
pub trait Command {
    fn execute(&self, db: &Db) -> Result<(), ()>;
}
