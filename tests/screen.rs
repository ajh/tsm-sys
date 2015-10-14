extern crate tsm_sys;
use tsm_sys::*;

#[test]
fn screen_creates_with_new() {
    let screen: Screen = Screen::new().unwrap();
}
