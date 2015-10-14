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
use ::screen::*;

// A trait that reads the output of the vte, and does whatever it wants with it.
pub trait VteReader {
    fn read(&mut self, String);
}

pub struct NullReader;

impl VteReader for NullReader {
    fn read(&mut self, _: String) {}
}

// A callback that puts the output in a string and sends it to the configured VteReader.
extern "C" fn callback<T: VteReader>(_: *mut tsm_vte, input_ptr: *const c_char, input_size: size_t, reader: *mut c_void) {
    let reader: &mut T = unsafe { &mut *(reader as *mut T) };

    //// Note: is it safe to put this stuff in a string? May it not be invalid unicode? Probably.
    let input = unsafe { slice::from_raw_parts(input_ptr, input_size as usize) };
    let input = input.iter().map(|c| *c as u8).collect();
    let string = String::from_utf8(input).unwrap();

    reader.read(string);
}

#[repr(C)]
pub struct Vte<T: VteReader> {
  pub vte: *mut tsm_vte,
  pub screen: *mut tsm_screen,
  pub reader: Box<T>,
}

impl<T: VteReader> Vte<T> {
    // Create a new Vte aka Virtual Terminal Emulator
    pub fn new(reader: T) -> Result<Vte<T>, String> {
        let mut screen = ptr::null_mut();
        let err = unsafe { tsm_screen_new(&mut screen, None, ptr::null_mut()) }; // ignore err for now

        let mut vte: Vte<T> = Vte {
            vte: ptr::null_mut(),
            screen: screen,
            reader: Box::new(reader),
        };

        let mut vte_ptr = ptr::null_mut();
        let err = unsafe {
            tsm_vte_new(
                &mut vte_ptr,
                screen,
                callback::<T>,
                &mut *vte.reader as *mut _ as *mut c_void,
                None,
                ptr::null_mut()
            )
        };
        vte.vte = vte_ptr;

        if err != 0 {
            return Err(format!("err is {}", err))
        }

        Ok(vte)
    }

    pub fn handle_keyboard(&self, c: char) {
        unsafe { tsm_vte_handle_keyboard(self.vte, 0, 0, 0, c as u32); };
    }

    // consider unicode here. I wonder what pty uses for the stream datatypes?
    //pub fn tty_input(&self, string: &str) {
        //unsafe { tsm_vte_input(self.vte, ); };
    //}
}

impl<T: VteReader> Drop for Vte<T> {
    fn drop(&mut self) {
        unsafe { tsm_vte_unref(self.vte) }
    }
}
