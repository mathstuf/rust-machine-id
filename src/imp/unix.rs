// Distributed under the OSI-approved BSD 3-Clause License.
// See accompanying LICENSE file for details.

use crates::uuid::{ParseError, Uuid};

use std::fs::File;
use std::io::{BufRead, BufReader};

error_chain! {
    errors {
        Io {
            description("I/O error")
        }

        Parse(err: ParseError) {
            description("parse error")
            display("parse error: {}", err)
        }
    }
}

fn get_machine_id_impl() -> Result<Uuid> {
    let fin = File::open("/etc/machine-id")
        .chain_err(|| ErrorKind::Io)?;
    let mut reader = BufReader::new(fin);
    let mut line = String::new();

    reader.read_line(&mut line)
        .chain_err(|| ErrorKind::Io)?;
    line.truncate(32);

    Uuid::parse_str(&line)
        .map_err(|err| ErrorKind::Parse(err).into())
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
