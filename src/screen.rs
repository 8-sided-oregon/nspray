use core::ffi::c_void;

use ndless_sys::{lcd_blit, lcd_init, scr_type_t};

const SCR_320X240_555: scr_type_t = 5;
const SCR_TYPE_INVALID: scr_type_t = 1;

pub fn init_screen() {
    let res = unsafe { lcd_init(SCR_320X240_555) };

    if !res {
        panic!("Failed to initialize screen");
    }
}

pub fn blit_buffer(buffer: &mut [u16]) {
    assert_eq!(buffer.len(), 320 * 240);

    unsafe {
        lcd_blit(buffer.as_mut_ptr() as *mut c_void, SCR_320X240_555);
    }
}

pub fn deinit_screen() {
    let res = unsafe { lcd_init(SCR_TYPE_INVALID) };

    if !res {
        panic!("Failed to deinitialize screen");
    }
}
