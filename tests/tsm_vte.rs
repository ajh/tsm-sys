#![feature(libc)]
extern crate libc;
extern crate regex;
extern crate tsm_sys;

use libc::c_char;
use libc::c_void;
use libc::size_t;
use std::ptr;
use std::slice;

use tsm_sys::*;

#[test]
fn tsm_vte_stuff_works() {
    let mut screen = ptr::null_mut();
    let err = unsafe { tsm::tsm_screen_new(&mut screen, None, ptr::null_mut()) };
    assert_eq!(0, err);

    let mut vte = ptr::null_mut();

    extern "C" fn write_cb(_: *mut tsm::TsmVte, input_ptr: *const c_char, input_size: size_t, output: *mut c_void) {
        let output: &mut Output = unsafe { &mut *(output as *mut Output) };
        let input = unsafe { slice::from_raw_parts(input_ptr, input_size as usize) };
        for c in input {
            output.string.push(*c as u8 as char);
        }
    }

    struct Output { string: String }
    let mut output = Output { string: "".to_string() };
    let output_ptr: *mut c_void = &mut output as *mut _ as *mut c_void;

    unsafe { tsm::tsm_vte_new(&mut vte, screen, write_cb, output_ptr, None, ptr::null_mut() ) };

    unsafe { tsm::tsm_vte_reset(vte) }
    unsafe { tsm::tsm_vte_hard_reset(vte) }

    for c in "hello world".chars() {
        unsafe { tsm::tsm_vte_handle_keyboard(vte, 0, 0, 0, c as u32); }
    }

    assert_eq!(&output.string, "hello world");
}
