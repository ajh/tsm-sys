use libc::{
    c_int,
    c_uint,
    c_void,
    size_t,
    uint32_t
};
use std::char::from_u32;
use std::ptr;

use ::tsm::*;

extern "C" fn draw_into_vector_callback(_: *mut TsmScreen,
                                           id: uint32_t,
                                           ch: *const uint32_t,
                                           len: size_t,
                                           width: c_uint,
                                           posx: c_uint,
                                           posy: c_uint,
                                           attr: *const TsmScreenAttr,
                                           age: TsmAge,
                                           vector: *mut c_void) -> c_int {
    let vector: &mut Vec<Cell> = unsafe { &mut *(vector as *mut Vec<Cell>) };
    let ch = unsafe { from_u32(*ch).unwrap() };
    let attr = unsafe { (*attr).clone() };

    let cell = Cell {
        id:     id as u32,
        ch:     ch,
        len:    len as usize,
        width:  width as usize,
        posx:   posx as usize,
        posy:   posy as usize,
        attr:   attr,
        age:    age as usize,
    };
    vector.push(cell);
    0
}

pub struct Cell {
    pub id: u32,
    pub ch: char,
    pub len: usize,
    pub width: usize,
    pub posx: usize,
    pub posy: usize,
    pub attr: TsmScreenAttr,
    pub age: usize,
}

pub struct Screen {
    // TODO: remove pub here once all api methods are implemented
    pub ptr: *mut TsmScreen,
}

impl Screen {
    pub fn new() -> Result<Screen, String> {
        let mut screen_ptr = ptr::null_mut();
        let result = unsafe { tsm_screen_new(&mut screen_ptr, None, ptr::null_mut()) };
        match result {
            0 => Ok(Screen { ptr: screen_ptr }),
            _ => Err(format!("error {}", result).to_string())
        }
    }

    pub fn resize(&mut self, row_size: u32, col_size: u32) -> Result<(), String> {
        unsafe {
            let result = tsm_screen_resize(self.ptr, row_size, col_size);
            match result {
                0 => Ok(()),
                _ => Err(format!("error {}", result).to_string())
            }
        }
    }

    pub fn get_width(&mut self) -> u32 {
        unsafe { tsm_screen_get_width(self.ptr) }
    }

    pub fn get_height(&mut self) -> u32 {
        unsafe { tsm_screen_get_height(self.ptr) }
    }

    // Could use std::io::Write trait but not sure how to handle attributes then.
    pub fn write(&mut self, c: char, attr: TsmScreenAttr) {
        unsafe { tsm_screen_write(self.ptr, c as u32, &attr); }
    }

    /// TODO: figure out how to also pass the TsmScreenAttr value. Having borrowing problems with
    /// that: "cannot move out of borrowed content [E0507]"
    pub fn draw<F>(&mut self, callback: F) where F: Fn(u32, char, usize, u32, u32, u32, TsmAge) {
        let callback_ptr = &callback as *const _ as *mut c_void;

        unsafe { tsm_screen_draw(self.ptr, draw_wrapper::<F>, callback_ptr); }

        // Shim interface function
        extern "C" fn draw_wrapper<F>(_: *mut TsmScreen,
                                      id: uint32_t,
                                      ch: *const uint32_t,
                                      len: size_t,
                                      width: c_uint,
                                      posx: c_uint,
                                      posy: c_uint,
                                      attr: *const TsmScreenAttr,
                                      age: TsmAge,
                                      closure: *mut c_void
                                      ) -> c_int
            where F: Fn(u32, char, usize, u32, u32, u32, TsmAge)
        {
            let closure = closure as *mut F;
            unsafe {
                let ch = from_u32(*ch).unwrap();

                (*closure)(id as u32,
                           ch as char,
                           len as usize,
                           width as u32,
                           posx as u32,
                           posy as u32,
                           age as TsmAge
                           )
            }

            0 as c_int
        }
    }

    // TODO: This is wasteful to build a new vec and cells everytime.
    pub fn cells(&mut self) -> Vec<Cell> {
        let mut cells = vec!();
        let cells_ptr: *mut c_void = &mut cells as *mut _ as *mut c_void;
        unsafe { tsm_screen_draw(self.ptr, draw_into_vector_callback, cells_ptr); }
        cells
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe { tsm_screen_unref(self.ptr) }
    }
}
