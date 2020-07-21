// Distributed under the OSI-approved BSD 3-Clause License.
// See accompanying LICENSE file for details.

use std::fmt;

use lazy_static::lazy_static;
use uuid::Uuid;

#[cfg(unix)]
mod imp {
    mod unix;
    pub use self::unix::get_machine_id;
}

#[cfg(windows)]
mod imp {
    mod windows;
    pub use self::windows::get_machine_id;
}

#[cfg(not(any(unix, windows)))]
mod imp {
    fn get_machine_id() -> Option<Uuid> {
        None
    }
}

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
            uuid: GLOBAL_ID.unwrap_or(*GENERATED_ID),
        }
    }
}

impl fmt::Display for MachineId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

impl From<MachineId> for Uuid {
    fn from(mid: MachineId) -> Uuid {
        mid.uuid
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
}
