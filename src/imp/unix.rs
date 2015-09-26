extern crate uuid;
use self::uuid::Uuid;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn _get_machine_id() -> Result<Uuid, Box<Error>> {
    let fin = try!(File::open("/etc/machine-id"));
    let mut reader = BufReader::new(fin);
    let mut line = String::new();

    try!(reader.read_line(&mut line));
    line.truncate(32);

    Ok(try!(Uuid::parse_str(&line)))
}

pub fn get_machine_id() -> Option<Uuid> {
    _get_machine_id().ok()
}
