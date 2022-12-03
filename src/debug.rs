// Tyler, if you're reading this, just... don't ask.

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

#[cfg(test)]
pub fn dprint_str(dstr: &str) {
    let nl = "\n";

    unsafe {
        write(1, dstr.as_ptr(), dstr.len());
        write(1, nl.as_ptr(), 1);
    }
}

#[cfg(not(test))]
pub fn dprint_str(dstr: &str) {}
