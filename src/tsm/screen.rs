use libc::{
    c_char,
    c_int,
    c_uint,
    c_void,
    int8_t,
    size_t,
    uint32_t,
    uint8_t
};

pub type TsmAge = uint32_t; // really a uint_fast32_t

pub enum TsmScreen {}

#[repr(C)]
pub struct TsmScreenAttr {
	fccode:  int8_t, 	// foreground color code or <0 for rgb
	bccode:  int8_t, 	// background color code or <0 for rgb
	fr:      uint8_t, 	// foreground red
	fg:      uint8_t, 	// foreground green
	fb:      uint8_t, 	// foreground blue
	br:      uint8_t, 	// background red
	bg:      uint8_t, 	// background green
	bb:      uint8_t, 	// background blue
    flags:   c_uint     // This is wrong. Need a c wrapper for the flags.
}

impl Default for TsmScreenAttr {
    fn default() -> TsmScreenAttr {
        TsmScreenAttr {
            fccode:  0,
            bccode:  0,
            fr:      0,
            fg:      0,
            fb:      0,
            br:      0,
            bg:      0,
            bb:      0,
            flags:   0
        }
    }
}

impl Clone for TsmScreenAttr {
    fn clone(&self) -> Self {
        TsmScreenAttr {
            fccode:  self.fccode,
            bccode:  self.bccode,
            fr:      self.fr,
            fg:      self.fg,
            fb:      self.fb,
            br:      self.br,
            bg:      self.bg,
            bb:      self.bb,
            flags:   self.flags,
        }
    }
}

#[repr(C)]
pub type DrawCallback = extern "C" fn(
    con:    *mut TsmScreen,
    id:     uint32_t,
    ch:     *const uint32_t,
    len:    size_t,
    width:  c_uint,
    posx:   c_uint,
    posy:   c_uint,
    attr:   *const TsmScreenAttr,
    age:    TsmAge,
    data:   *mut c_void
    ) -> c_int;

#[repr(C)]
pub type TsmSymbol = uint32_t;

extern {
  pub fn tsm_screen_new(out: *mut *mut TsmScreen,
                        log: Option<::tsm::tsm_log_t>,
                        log_data: *mut c_void) -> c_int;
  pub fn tsm_screen_ref(con: *mut TsmScreen);
  pub fn tsm_screen_unref(con: *mut TsmScreen);
  pub fn tsm_screen_get_width(con: *mut TsmScreen) -> c_uint;
  pub fn tsm_screen_get_height(con: *mut TsmScreen) -> c_uint;
  pub fn tsm_screen_resize(con: *mut TsmScreen, x: c_uint, y: c_uint) -> c_uint;
  pub fn tsm_screen_set_margins(con: *mut TsmScreen, top: c_uint, bottom: c_uint) -> c_uint;
  pub fn tsm_screen_set_max_sb(con: *mut TsmScreen, top: c_uint) -> c_uint;
  pub fn tsm_screen_clear_sb(con: *mut TsmScreen);
  pub fn tsm_screen_sb_up(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_sb_down(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_sb_page_up(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_sb_page_down(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_sb_reset(con: *mut TsmScreen);
  pub fn tsm_screen_set_def_attr(con: *mut TsmScreen, attr: *const TsmScreenAttr);
  pub fn tsm_screen_reset(con: *mut TsmScreen);
  pub fn tsm_screen_set_flags(con: *mut TsmScreen, flags: c_uint);
  pub fn tsm_screen_reset_flags(con: *mut TsmScreen, flags: c_uint);
  pub fn tsm_screen_get_flags(con: *mut TsmScreen) -> c_uint;
  pub fn tsm_screen_get_cursor_x(con: *mut TsmScreen) -> c_uint;
  pub fn tsm_screen_get_cursor_y(con: *mut TsmScreen) -> c_uint;
  pub fn tsm_screen_set_tabstop(con: *mut TsmScreen);
  pub fn tsm_screen_reset_tabstop(con: *mut TsmScreen);
  pub fn tsm_screen_reset_all_tabstop(con: *mut TsmScreen);
  pub fn tsm_screen_write(con: *mut TsmScreen, ch: TsmSymbol, attr: *const TsmScreenAttr);
  pub fn tsm_screen_newline(con: *mut TsmScreen);
  pub fn tsm_screen_scroll_up(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_scroll_down(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_move_to(con: *mut TsmScreen, x: c_uint, y: c_uint);
  pub fn tsm_screen_move_up(con: *mut TsmScreen, num: c_uint, scroll: bool);
  pub fn tsm_screen_move_down(con: *mut TsmScreen, num: c_uint, scroll: bool);
  pub fn tsm_screen_move_left(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_move_right(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_move_line_end(con: *mut TsmScreen);
  pub fn tsm_screen_move_line_home(con: *mut TsmScreen);
  pub fn tsm_screen_tab_right(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_tab_left(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_insert_lines(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_delete_lines(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_insert_chars(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_delete_chars(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_erase_cursor(con: *mut TsmScreen);
  pub fn tsm_screen_erase_cursor_chars(con: *mut TsmScreen, num: c_uint);
  pub fn tsm_screen_erase_cursor_to_end(con: *mut TsmScreen, protect: bool);
  pub fn tsm_screen_erase_home_to_cursor(con: *mut TsmScreen, protect: bool);
  pub fn tsm_screen_erase_current_line(con: *mut TsmScreen, protect: bool);
  pub fn tsm_screen_erase_screen_to_cursor(con: *mut TsmScreen, protect: bool);
  pub fn tsm_screen_erase_cursor_to_screen(con: *mut TsmScreen, protect: bool);
  pub fn tsm_screen_erase_screen(con: *mut TsmScreen, protect: bool);
  pub fn tsm_screen_selection_reset(con: *mut TsmScreen);
  pub fn tsm_screen_selection_start(con: *mut TsmScreen, posx: c_uint, posy: c_uint);
  pub fn tsm_screen_selection_target(con: *mut TsmScreen, posx: c_uint, posy: c_uint);
  pub fn tsm_screen_selection_copy(con: *mut TsmScreen, out: *mut *mut c_char) -> c_int;
  pub fn tsm_screen_draw(con: *mut TsmScreen, draw_cb: DrawCallback, data: *mut c_void) -> c_int;
}
