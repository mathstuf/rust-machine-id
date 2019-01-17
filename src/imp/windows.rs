// Distributed under the OSI-approved BSD 3-Clause License.
// See accompanying LICENSE file for details.

use crates::advapi32::GetCurrentHwProfileA;
use crates::uuid::Uuid;
use crates::winapi::winbase::HW_PROFILE_INFOA;
use crates::win32_error::Win32Error;

error_chain! {
    foreign_links {
        Win32(win32_error::Win32Error)
            #[doc = "An error from Windows."];
    }

    errors {
        Parse(err: ParseError) {
            description("parse error")
            display("parse error: {}", err)
        }
    }
}

fn get_machine_id_impl() -> Result<Uuid> {
    let profile_info: HW_PROFILE_INFOA;
    let have_profile = GetCurrentHwProfileA(&mut profile_info);

    if have_profile {
        Uuid::parse_str(&profile_info.szHwProfileGuid[1..37])
            .map_err(|err| ErrorKind::Parse(err).into())
    } else {
        Err(Win32Error::new().into())
    }
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
