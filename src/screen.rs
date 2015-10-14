use libc::c_char;
use libc::c_int;
use libc::c_uint;
use libc::c_void;
use libc::size_t;
use libc::uint32_t;
use std::char::from_u32;
use std::default;
use std::fmt;
use std::ptr;
use std::slice;
use std::char;
use std::result::Result;

use ::tsm::*;

pub struct Screen {
    pub screen: *mut tsm_screen,
}

impl Screen {
    // Create a new Screen
    pub fn new() -> Result<Screen, String> {
        let mut screen = ptr::null_mut();
        let err = unsafe { tsm_screen_new(&mut screen, None, ptr::null_mut()) };

        match err {
            0 => Ok(Screen { screen: screen }),
            _ => Err(format!("err is {}", err)),
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe { tsm_screen_unref(self.screen) }
    }
}
