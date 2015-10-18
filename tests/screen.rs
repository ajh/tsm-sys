#![feature(libc)]
extern crate libc;
extern crate regex;
extern crate tsm_sys;

use regex::Regex;
use std::char;
use std::sync::{Arc, Mutex};

use tsm_sys::*;

#[test]
fn screen_stuff_works() {
    let mut screen = Screen::new().unwrap();
    screen.resize(80, 24).unwrap();
    assert_eq!(80, screen.get_width());
    assert_eq!(24, screen.get_height());

    for c in "hello world".chars() {
        let attr: tsm::TsmScreenAttr = Default::default();
        screen.write(c, attr);
    }

    let output = Arc::new(Mutex::new("".to_string()));

    screen.draw(|_, ch, _, _, _, _, _| {
        output.lock().unwrap().push(ch);
    });

    let re = Regex::new(r"hello world").unwrap();
    assert!(re.is_match(&(output.lock().unwrap())));
}

fn screen_returns_vec_of_cells() {
    let mut screen = Screen::new().unwrap();
    screen.resize(80, 24).unwrap();
    assert_eq!(80, screen.get_width());
    assert_eq!(24, screen.get_height());

    let mut output = String::new();
    for cell in screen.cells() {
        output.push(cell.ch);
    }

    let re = Regex::new(r"hello world").unwrap();
    assert!(re.is_match(&(output)));
}
