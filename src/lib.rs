// Distributed under the OSI-approved BSD 3-Clause License.
// See accompanying LICENSE file for details.

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod crates {
    pub extern crate uuid;

    #[cfg(windows)]
    pub extern crate advapi32;
    #[cfg(windows)]
    pub extern crate winapi;
    #[cfg(windows)]
    pub extern crate win32_error;
}

use crates::uuid::{Uuid, UuidVersion};

use std::fmt;

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
    static ref GENERATED_ID: Uuid = Uuid::new(UuidVersion::Random)
        .expect("failed to generate a random uuid");
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
