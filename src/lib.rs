#[macro_use]
extern crate lazy_static;

extern crate uuid;
use self::uuid::Uuid;

mod imp {
    fn get_machine_id() -> Option<Uuid> {
        None
    }
}

use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MachineId {
    uuid: Uuid,
}

lazy_static! {
    static ref GENERATED_ID: Uuid = Uuid::new_v4();
    static ref GLOBAL_ID: Option<Uuid> = imp::get_machine_id();
}

impl MachineId {
    pub fn new() -> MachineId {
        MachineId {
            uuid: GLOBAL_ID.unwrap_or(*GENERATED_ID).clone(),
        }
    }
}

impl fmt::Display for MachineId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uuid)
    }
}
