#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate uuid;
use self::uuid::Uuid;

#[cfg(unix)]
mod imp {
    mod unix;
    pub use self::unix::get_machine_id;
}

#[cfg(not(any(unix)))]
mod imp {
    fn get_machine_id() -> Option<Uuid> {
        None
    }
}

use std::fmt;

/// A machine-specific ID.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MachineId {
    uuid: Uuid,
}

lazy_static! {
    static ref GENERATED_ID: Uuid = Uuid::new_v4();
    static ref GLOBAL_ID: Option<Uuid> = imp::get_machine_id();
}

impl MachineId {
    /// Retrieves or generates the machine-specific ID.
    pub fn get() -> MachineId {
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

#[test]
fn test_idempotent() {
    let fst = MachineId::get();
    let snd = MachineId::get();

    assert_eq!(fst, snd);
}

#[test]
fn test_can_print() {
    let mid = MachineId::get();

    println!("{}", mid);
    assert_eq!(false, true);
}
