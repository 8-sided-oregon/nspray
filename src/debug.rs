// Tyler, if you're reading this, just... don't ask.

use crate::{LOG_FILE, START_TIME};
use alloc::format;
use ndless::{io::Write, time::SystemTime};

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

// #[allow(dead_code)]
// #[cfg(test)]
// pub fn dprint_str(dstr: &str) {
//     let nl = "\n";

//     unsafe {
//         write(1, dstr.as_ptr(), dstr.len());
//         write(1, nl.as_ptr(), 1);
//     }
// }

pub fn dprint_str(dstr: &str) {
    let file = unsafe { LOG_FILE.as_mut() }.unwrap();
    let duration = unsafe { START_TIME.as_ref().unwrap().clone() };

    let msg = format!(
        "[{:5}]: {}\n",
        SystemTime::now()
            .duration_since(duration)
            .expect("Failed to take time measurement")
            .as_secs(),
        dstr,
    );

    file.write_all(msg.as_bytes())
        .expect("Failed logging message");
}
