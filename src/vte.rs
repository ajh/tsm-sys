use libc::c_char;
use libc::c_void;
use libc::size_t;
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;
use std::slice;
use std::sync::mpsc::{Sender, Receiver, channel};

use ::tsm::*;

pub struct Vte {
    // TODO: remove pub here once all api methods are implemented
    pub ptr: *mut TsmVte,

    // I'm trying to use the rust feature that best reflects the reality that libtsm's vte ptr
    // mutates the screen in c-land.
    pub screen: Rc<RefCell<::Screen>>,
    tx: Box<Sender<char>>,
    pub rx: Receiver<char>,
}

/// a callback that sends the a channel passed as a pointer
extern "C" fn callback(_: *mut TsmVte, input_ptr: *const c_char, input_size: size_t, tx_channel: *mut c_void) {
    let tx_channel: &mut Sender<char> = unsafe { &mut *(tx_channel as *mut Sender<char>) };
    let input = unsafe { slice::from_raw_parts(input_ptr, input_size as usize) };
    for c in input {
        // TODO: don't unwrap here!
        tx_channel.send(*c as u8 as char).unwrap();
    }
}

impl Vte {
    pub fn new(rows_count: usize, cols_count: usize) -> Result<Vte, String> {
        let screen = ::Screen::new(rows_count, cols_count).unwrap();
        let (tx, rx) = channel();
        let mut boxed_tx = Box::new(tx); // stablize memory address of tx

        let mut vte_ptr = ptr::null_mut();
        let tx_pointer: *mut c_void = &mut *boxed_tx as *mut _ as *mut c_void;

        // no result? really?
        unsafe { tsm_vte_new(&mut vte_ptr, screen.ptr, callback, tx_pointer, None, ptr::null_mut() ) };

        let vte = Vte {
            ptr: vte_ptr,
            screen: Rc::new(RefCell::new(screen)),
            tx: boxed_tx,
            rx: rx,
        };

        Ok(vte)
    }

    pub fn handle_keyboard(&mut self, ch: char) {
        unsafe { tsm_vte_handle_keyboard(self.ptr, 0, 0, 0, ch as u32); }
    }

    // Not even sure what this does. Is it a sink for the ptys output?
    pub fn input(&mut self, bytes: &[u8]) {
        unsafe { tsm_vte_input(self.ptr, bytes.as_ptr(), bytes.len() as size_t ); }
    }
}

impl Drop for Vte {
    fn drop(&mut self) {
        unsafe { tsm_vte_unref(self.ptr) }
    }
}
