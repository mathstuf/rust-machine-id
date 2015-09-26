extern crate uuid;
use self::uuid::Uuid;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn _get_machine_id() -> io::Result<Uuid> {
    let fin = try!(File::open("/etc/machine-id"));
    let mut reader = BufReader::new(fin);
    let mut line = String::new();

    try!(reader.read_line(&mut line));

    match Uuid::parse_str(&line) {
        Ok(uuid) => Ok(uuid),
        Err(_)   => Err(io::Error::last_os_error()),
    }
}

pub fn get_machine_id() -> Option<Uuid> {
    _get_machine_id().ok()
}
