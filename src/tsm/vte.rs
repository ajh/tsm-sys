use libc::{
    c_char,
    c_int,
    c_uint,
    c_void,
    size_t,
    uint32_t
};

pub enum TsmVte {}

#[repr(C)]
pub type tsm_vte_write_cb = extern "C" fn(
    vte: *mut TsmVte,
    u8: *const c_char,
    len: size_t,
    data: *mut c_void
);

extern {
    pub fn tsm_vte_new(out: *mut *mut TsmVte,
                       con: *mut ::tsm::TsmScreen,
                       write_cb: tsm_vte_write_cb,
                       data: *mut c_void,
                       log: Option<::tsm::tsm_log_t>,
                       log_data: *mut c_void) -> c_int;

    pub fn tsm_vte_ref(vte: *mut TsmVte);
    pub fn tsm_vte_unref(vte: *mut TsmVte);
    pub fn tsm_vte_set_palette(vte: *mut TsmVte, palette: *const c_char);
    pub fn tsm_vte_reset(vte: *mut TsmVte);
    pub fn tsm_vte_hard_reset(vte: *mut TsmVte);
    pub fn tsm_vte_input(vte: *mut TsmVte, u8: *const u8, len: size_t);
    pub fn tsm_vte_handle_keyboard(vte: *mut TsmVte,
                                   keysym: uint32_t,
                                   ascii: uint32_t,
                                   mods: c_uint,
                                   unicode: uint32_t) -> bool;
}
