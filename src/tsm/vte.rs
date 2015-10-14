use libc::c_int;
use libc::c_char;
use libc::c_uint;
use libc::c_void;
use libc::size_t;
use libc::uint32_t;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct tsm_vte;

#[repr(C)]
pub type tsm_vte_write_cb = extern "C" fn(
    vte: *mut tsm_vte,
    u8: *const c_char,
    len: size_t,
    data: *mut c_void
    );

extern {
    pub fn tsm_vte_new(out: *mut *mut tsm_vte,
                       con: *mut ::tsm::tsm_screen,
                       write_cb: tsm_vte_write_cb,
                       data: *mut c_void,
                       log: Option<::tsm::tsm_log_t>,
                       log_data: *mut c_void) -> c_int;

    pub fn tsm_vte_ref(vte: *mut tsm_vte);
    pub fn tsm_vte_unref(vte: *mut tsm_vte);
    pub fn tsm_vte_set_palette(vte: *mut tsm_vte, palette: *const c_char);
    pub fn tsm_vte_reset(vte: *mut tsm_vte);
    pub fn tsm_vte_hard_reset(vte: *mut tsm_vte);
    pub fn tsm_vte_input(vte: *mut tsm_vte, u8: *const c_char, len: size_t);
    pub fn tsm_vte_handle_keyboard(vte: *mut tsm_vte,
                                   keysym: uint32_t,
                                   ascii: uint32_t,
                                   mods: c_uint,
                                   unicode: uint32_t) -> bool;
}
