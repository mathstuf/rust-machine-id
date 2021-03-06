// Distributed under the OSI-approved BSD 3-Clause License.
// See accompanying LICENSE file for details.

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use log::debug;
use thiserror::Error;
use uuid::{self, Uuid};

#[derive(Debug, Error)]
enum Error {
    #[error("I/O error")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("parse error")]
    Parse {
        #[from]
        source: uuid::Error,
    },
}

fn get_machine_id_impl() -> Result<Uuid, Error> {
    let fin = File::open("/etc/machine-id")?;
    let mut reader = BufReader::new(fin);
    let mut line = String::new();

    reader.read_line(&mut line)?;
    line.truncate(32);

    Ok(Uuid::parse_str(&line)?)
}

pub fn get_machine_id() -> Option<Uuid> {
    match get_machine_id_impl() {
        Ok(uuid) => Some(uuid),
        Err(err) => {
            debug!("{:?}", err);
            None
        },
    }
}
