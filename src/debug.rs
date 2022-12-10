// Tyler, if you're reading this, just... don't ask.

use crate::LOG_FILE;
use ndless::io::Write;

#[cfg(test)]
extern "C" {
    fn write(fd: i32, buf: *const u8, count: usize) -> i32;
}

#[macro_export]
macro_rules! dprintln {
    ($($element:expr),*) => {
        $crate::debug::dprint_str(&alloc::format!($($element),*));
    };
}

#[allow(dead_code)]
#[cfg(test)]
pub fn dprint_str(dstr: &str) {
    let nl = "\n";

    unsafe {
        write(1, dstr.as_ptr(), dstr.len());
        write(1, nl.as_ptr(), 1);
    }
}

#[cfg(not(test))]
pub fn dprint_str(dstr: &str) {
    let file = unsafe { LOG_FILE.as_mut() }.unwrap();

    file.write_all(dstr.as_bytes())
        .expect("Failed to log message");

    file.write_all(b"\n").expect("Failed to write newline");
}
