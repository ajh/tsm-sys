extern crate tsm_sys;
use tsm_sys::*;

struct TestReader { s: String, }

impl VteReader for TestReader {
    fn read(&mut self, val: String) { self.s.push_str(&val); }
}

#[test]
fn vte_creates_with_new() {
    let vte = Vte::new(NullReader).unwrap();
}

#[test]
fn vte_handles_user_keys() {
    let mut vte = Vte::new(NullReader).unwrap();

    for c in "hello world".chars() {
        vte.handle_keyboard(c);
    }
}

#[test]
fn vte_calls_reader_when_writing() {
    let vte = Vte::new(TestReader { s: "".to_string() }).unwrap();
    for c in "hello world".chars() {
        vte.handle_keyboard(c);
    }

    assert_eq!(vte.reader.s, "hello world");
}

//#[test]
//fn vte_handles_tty_input() {
    //let vte = Vte::new(TestReader { s: "".to_string() }).unwrap();
    //for c in "\x1b[10;1Hhello world".chars() {
        //vte.tty_input(c);
    //}

    //assert_eq!(vte.reader.s, "hello world");
//}
//fn vte_calls_reader_when_writing_based_on_tty_input() {
