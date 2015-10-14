use libc::c_int;
use libc::c_char;
use libc::c_uint;
use libc::c_void;
use libc::int8_t;
use libc::size_t;
use libc::uint32_t;
use libc::uint8_t;

pub type tsm_age_t = uint32_t; // really a uint_fast32_t

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct tsm_screen;

#[repr(C)]
pub struct tsm_screen_attr {
	fccode: int8_t, 	// foreground color code or <0 for rgb
	bccode: int8_t, 	// background color code or <0 for rgb
	fr:	   uint8_t, 	// foreground red
	fg:	   uint8_t, 	// foreground green
	fb:	   uint8_t, 	// foreground blue
	br:	   uint8_t, 	// background red
	bg:	   uint8_t, 	// background green
	bb:	   uint8_t, 	// background blue
    flags: c_uint       // This is wrong. Need a c wrapper for the flags.
}
impl Default for tsm_screen_attr {
    fn default() -> tsm_screen_attr {
        tsm_screen_attr {
            fccode: 0,
            bccode: 0,
            fr: 0,
            fg: 0,
            fb: 0,
            br: 0,
            bg: 0,
            bb: 0,
            flags: 0
        }
    }
}

#[repr(C)]
pub type tsm_screen_draw_cb = extern "C" fn(
    con: *mut tsm_screen,
    id: uint32_t,
    ch: *const uint32_t,
    len: size_t,
    width: c_uint,
    posx: c_uint,
    posy: c_uint,
    attr: *const tsm_screen_attr,
    age: tsm_age_t,
    data: *mut c_void
    ) -> c_int;

#[repr(C)]
pub type tsm_symbol_t = uint32_t;

extern {
  pub fn tsm_screen_new(out: *mut *mut tsm_screen,
                        log: Option<::tsm::tsm_log_t>,
                        log_data: *mut c_void) -> c_int;
  pub fn tsm_screen_ref(con: *mut tsm_screen);
  pub fn tsm_screen_unref(con: *mut tsm_screen);
  pub fn tsm_screen_get_width(con: *mut tsm_screen) -> c_uint;
  pub fn tsm_screen_get_height(con: *mut tsm_screen) -> c_uint;
  pub fn tsm_screen_resize(con: *mut tsm_screen, x: c_uint, y: c_uint) -> c_uint;
  pub fn tsm_screen_set_margins(con: *mut tsm_screen, top: c_uint, bottom: c_uint) -> c_uint;
  pub fn tsm_screen_set_max_sb(con: *mut tsm_screen, top: c_uint) -> c_uint;
  pub fn tsm_screen_clear_sb(con: *mut tsm_screen);
  pub fn tsm_screen_sb_up(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_sb_down(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_sb_page_up(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_sb_page_down(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_sb_reset(con: *mut tsm_screen);
  pub fn tsm_screen_set_def_attr(con: *mut tsm_screen, attr: *const tsm_screen_attr);
  pub fn tsm_screen_reset(con: *mut tsm_screen);
  pub fn tsm_screen_set_flags(con: *mut tsm_screen, flags: c_uint);
  pub fn tsm_screen_reset_flags(con: *mut tsm_screen, flags: c_uint);
  pub fn tsm_screen_get_flags(con: *mut tsm_screen) -> c_uint;
  pub fn tsm_screen_get_cursor_x(con: *mut tsm_screen) -> c_uint;
  pub fn tsm_screen_get_cursor_y(con: *mut tsm_screen) -> c_uint;
  pub fn tsm_screen_set_tabstop(con: *mut tsm_screen);
  pub fn tsm_screen_reset_tabstop(con: *mut tsm_screen);
  pub fn tsm_screen_reset_all_tabstop(con: *mut tsm_screen);
  pub fn tsm_screen_write(con: *mut tsm_screen, ch: tsm_symbol_t, attr: *const tsm_screen_attr);
  pub fn tsm_screen_newline(con: *mut tsm_screen);
  pub fn tsm_screen_scroll_up(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_scroll_down(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_move_to(con: *mut tsm_screen, x: c_uint, y: c_uint);
  pub fn tsm_screen_move_up(con: *mut tsm_screen, num: c_uint, scroll: bool);
  pub fn tsm_screen_move_down(con: *mut tsm_screen, num: c_uint, scroll: bool);
  pub fn tsm_screen_move_left(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_move_right(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_move_line_end(con: *mut tsm_screen);
  pub fn tsm_screen_move_line_home(con: *mut tsm_screen);
  pub fn tsm_screen_tab_right(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_tab_left(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_insert_lines(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_delete_lines(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_insert_chars(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_delete_chars(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_erase_cursor(con: *mut tsm_screen);
  pub fn tsm_screen_erase_cursor_chars(con: *mut tsm_screen, num: c_uint);
  pub fn tsm_screen_erase_cursor_to_end(con: *mut tsm_screen, protect: bool);
  pub fn tsm_screen_erase_home_to_cursor(con: *mut tsm_screen, protect: bool);
  pub fn tsm_screen_erase_current_line(con: *mut tsm_screen, protect: bool);
  pub fn tsm_screen_erase_screen_to_cursor(con: *mut tsm_screen, protect: bool);
  pub fn tsm_screen_erase_cursor_to_screen(con: *mut tsm_screen, protect: bool);
  pub fn tsm_screen_erase_screen(con: *mut tsm_screen, protect: bool);
  pub fn tsm_screen_selection_reset(con: *mut tsm_screen);
  pub fn tsm_screen_selection_start(con: *mut tsm_screen, posx: c_uint, posy: c_uint);
  pub fn tsm_screen_selection_target(con: *mut tsm_screen, posx: c_uint, posy: c_uint);
  pub fn tsm_screen_selection_copy(con: *mut tsm_screen, out: *mut *mut c_char) -> c_int;
  pub fn tsm_screen_draw(con: *mut tsm_screen, draw_cb: tsm_screen_draw_cb, data: *mut c_void) -> c_int;
}
