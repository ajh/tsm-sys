#![feature(libc)]
extern crate libc;
extern crate regex;
extern crate tsm_sys;

use libc::c_char;
use libc::c_int;
use libc::c_uint;
use libc::c_void;
use libc::size_t;
use libc::uint32_t;
use regex::Regex;
use std::char::from_u32;
use std::default;
use std::fmt;
use std::ptr;
use std::slice;
use std::char;

use tsm_sys::*;

#[test]
fn tsm_screen_stuff_works() {
    let mut screen = ptr::null_mut();
    let err = unsafe { tsm::tsm_screen_new(&mut screen, None, ptr::null_mut()) };
    assert_eq!(0, err);

    let err = unsafe { tsm::tsm_screen_resize(screen, 80, 24) };
    assert_eq!(0, err);
    assert_eq!(80, unsafe { tsm::tsm_screen_get_width(screen) });
    assert_eq!(24, unsafe { tsm::tsm_screen_get_height(screen) });

    let attr: tsm::tsm_screen_attr = Default::default();
    for c in "hello world".chars() {
        unsafe { tsm::tsm_screen_write(screen, c as u32, &attr); }
    }
    extern "C" fn draw_cb(_: *mut tsm::tsm_screen, _: u32, ch: *const uint32_t, _: size_t, _: c_uint, _: c_uint, _: c_uint, _: *const tsm::tsm_screen_attr, _: tsm::tsm_age_t, output: *mut c_void) -> c_int {
        let output: &mut Output = unsafe { &mut *(output as *mut Output) };
        let char = unsafe {
            if *ch == 0 { ' ' } else { from_u32(*ch).unwrap() }
        };
        output.string.push(char);
        0
    }

    struct Output { string: String }
    let mut output = Output { string: "".to_string() };
    let output_ptr: *mut c_void = &mut output as *mut _ as *mut c_void;
    unsafe { tsm::tsm_screen_draw(screen, draw_cb, output_ptr) };
    let re = Regex::new(r"hello world").unwrap();
    assert!(re.is_match(&output.string));
}
