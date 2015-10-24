extern crate regex;
extern crate tsm_sys;

use regex::Regex;
use std::sync::{Arc, Mutex};

use tsm_sys::*;

#[test]
fn vte_stuff_works() {
    let mut vte = Vte::new(10, 1).unwrap();

    for c in "hello world".chars() {
        vte.handle_keyboard(c);
    }

    let mut output = "".to_string();

    loop {
        match vte.rx.try_recv() {
            Ok(ch) => output.push(ch),
            Err(_) => break
        }
    }

    assert_eq!(output, "hello world");
}

#[test]
fn vte_new_creates_a_new_vte() {
    Vte::new(2, 2).unwrap();
}

#[test]
fn vte_allows_mutable_access_to_its_screen() {
    let mut vte = Vte::new(2, 2).unwrap();
    {
        let mut screen = vte.screen.borrow_mut();
        screen.resize(3, 1).unwrap();
    }

    // this doesn't explicitly check that screen is shared properly, but seems to work for now.
    vte.handle_keyboard('h' as char);
}

#[test]
fn vte_input_changes_the_screen() {
    let mut vte = Vte::new(10, 2).unwrap();

    vte.input(b"hello world");

    let output = Arc::new(Mutex::new("".to_string()));

    {
        let mut screen = vte.screen.borrow_mut();

        screen.draw(|_, ch, _, _, _, _, _| {
            output.lock().unwrap().push(ch);
        });
    }

    let re = Regex::new(r"hello world").unwrap();
    assert!(re.is_match(&(output.lock().unwrap())));
}
