use crate::{IMG_HEIGHT, IMG_WIDTH};
use alloc::vec;

/// Dithers the input array (RGB888) and puts the output into the output array (RGB555).
/// This uses the `minimized average error` kernel, which is:
/// 1/48 * [ _ _ # 7 5 ]
///        [ 3 5 7 5 3 ]
///        [ 1 3 5 3 1 ]
pub fn dither(in_img: &[u8], out_img: &mut [u16]) {
    let kernel = [[0i16, 0, 0, 7, 5], [3i16, 5, 7, 5, 3], [1i16, 3, 5, 3, 1]];

    for i in 0..(IMG_HEIGHT * IMG_WIDTH) {
        out_img[i] = 0;
    }

    // 0 -> R, 1 -> G, 2 -> B
    for c in 0..3 {
        let mut error_buff = vec![0i16; IMG_WIDTH * IMG_HEIGHT * 3];

        for i in 0..IMG_HEIGHT {
            for j in 0..IMG_WIDTH {
                let in_index = (i * IMG_WIDTH + j) * 3;
                let out_index = i * IMG_WIDTH + j;

                let color = in_img[in_index + c] as i16;
                let converted_color =
                    (((color + error_buff[out_index]) * 0x1f) / 0xff).clamp(0, 0x1f);
                let error = (converted_color * 0xff / 0x1f) - color;

                out_img[out_index] |= (converted_color as u16) << ((2 - c) * 5);

                for k in 0..3 {
                    for n in 0..5 {
                        if k == 0 && n < 3 {
                            continue;
                        }

                        if k + i >= IMG_HEIGHT {
                            continue;
                        }

                        if (j as isize + n as isize - 2) < 0 || n + j >= IMG_WIDTH {
                            continue;
                        }

                        error_buff[out_index + k * IMG_WIDTH + n - 2] -= error * kernel[k][n] / 48;
                    }
                }
            }
        }
    }
}
