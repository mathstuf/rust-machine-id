// Distributed under the OSI-approved BSD 3-Clause License.
// See accompanying LICENSE file for details.

// Implemented by JR Andreassen jr at prtgconsultants com
// references:
//   https://github.com/denisbrodbeck/machineid
//   https://github.com/jojonv/rust-win32error/blob/master/src/lib.rs

use std::io;

use log::debug;
use rust_win32error::Win32Error;
use thiserror::Error;
use uuid::{self, Uuid};
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

#[derive(Debug, Error)]
enum Error {
    #[error("I/O error")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("W32 error")]
    W32Err {
        #[from]
        source: rust_win32error::Win32Error,
    },
    #[error("parse error")]
    Parse {
        #[from]
        source: uuid::Error,
    },
}

const WIN_REG_BASE_KEY: &'static str = r#"SOFTWARE\Microsoft\Cryptography"#;
const WIN_REG_BASE_VALUE: &'static str = "MachineGuid";

fn get_machine_id_impl(key_path: &str, key_name: &str) -> Result<Uuid, Error> {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = match hklm.open_subkey_with_flags(key_path, KEY_READ) {
        Ok(val) => val,
        Err(err) => {
            debug!(
                "get_machine_id_impl: Failed to read machine UUID W32 {:?}",
                err,
            );
            return Err(err.into());
        },
    };

    let machine_uuid = match subkey.get_value(key_name) {
        Ok(value) => value,
        Err(err) => {
            let w32err = Win32Error::new();
            if w32err.get_error_code() != 0 {
                debug!(
                    "get_machine_id_impl: Failed to read machine UUID W32 {:?}",
                    w32err,
                );
                return Err(w32err.into());
            }
            debug!(
                "get_machine_id_impl: Failed to read machine UUID W32 {:?}",
                err,
            );
            return Err(err.into());
        },
    };

    Ok(Uuid::parse_str(&machine_uuid)?)
}

pub fn get_machine_id() -> Option<Uuid> {
    match get_machine_id_impl(WIN_REG_BASE_KEY, WIN_REG_BASE_VALUE) {
        Ok(uuid) => Some(uuid),
        Err(err) => {
            debug!("get_machine_id: Impl returned error {:?}", err);
            None
        },
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_get_machine_id_impl() {
        let first = match get_machine_id_impl(WIN_REG_BASE_KEY, WIN_REG_BASE_VALUE) {
            Ok(uuid) => uuid,
            Err(err) => {
                panic!("test_get_machine_id_impl: First Get Failed: {:?}", err);
            },
        };
        let second = match get_machine_id_impl(WIN_REG_BASE_KEY, WIN_REG_BASE_VALUE) {
            Ok(uuid) => uuid,
            Err(err) => {
                panic!("test_get_machine_id_impl: Second Get Failed: {:?}", err);
            },
        };
        println!(
            "test_get_machine_id_impl: [1]{{{:?}}}, [2]{{{:?}}}",
            first, second,
        );
        assert_eq!(first, second);
    }

    #[test]
    fn test_get_machine_id_impl_fail_path() {
        let path = r#"SOFTWARE\\Microsoft\\Junk"#;
        let _ = match get_machine_id_impl(path, WIN_REG_BASE_VALUE) {
            Ok(uuid) => {
                panic!(
                    "test_get_machine_id_impl: Should have failed on path[{}]: Got::{:?}",
                    path, uuid,
                );
            },
            Err(err) => {
                println!(
                    "test_get_machine_id_impl: First Get [{}] Failed: {:?}",
                    path, err,
                );
                Uuid::nil()
            },
        };
    }

    #[test]
    fn test_get_machine_id_impl_fail_key() {
        let key = "BahhHumbug";
        let _ = match get_machine_id_impl(WIN_REG_BASE_KEY, key) {
            Ok(uuid) => {
                panic!(
                    "test_get_machine_id_impl: Should have failed on key[{}]: Got::{:?}",
                    key, uuid,
                );
            },
            Err(err) => {
                println!(
                    "test_get_machine_id_impl: Second Get[{}] Failed: {:?}",
                    key, err,
                );
                Uuid::nil()
            },
        };
    }

    #[test]
    fn test_get_machine_id() {
        if let Some(uuid) = get_machine_id() {
            println!("test_get_machine_id: {{{:?}}}", uuid);
        } else {
            panic!("get_machine_id Failed");
        }
    }
}
