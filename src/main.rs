//#![cfg_attr(not(test), no_std)]

#![no_std]
#![feature(format_args_capture)]

//#![cfg_attr(not(feature = "std"), no_std)]

/* Hello! Welcome to my cool raytracer program!
 *
 * If for some extremely strange reason you wish to modify this code or take note of it, please
 * note the following quirks and things that I've discovered:
 *
 * # You cannot use the entire range of usable memory if you try and allocate it with arrays
 *   (e.g. [0u8; 1024 * 1024 * 1024 * 3]). If you try and do that, the calculator hangs. Instead,
 *   you have to use Vectors and dynamic allocation.
 */

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use camera::Camera;
use caster::Renderer;
use hittable::{Hittable, HittableList, Sphere};
use material::{Lambertian, Metal};
use ndless::{fs::File, input::wait_key_pressed, io::BufWriter};
use screen::{blit_buffer, deinit_screen, init_screen};
use vec3::Vec3FI32;

extern crate ndless;
extern crate ndless_sys;

#[cfg(not(test))]
extern crate ndless_handler;

extern crate alloc;
extern crate oorandom;

mod camera;
mod caster;
mod debug;
mod dither;
mod fixed;
mod hittable;
mod material;
mod ray;
mod screen;
mod tests;
mod vec3;

const IMG_WIDTH: usize = 320;
const IMG_HEIGHT: usize = 240;

// This is a really bad idea
static mut LOG_FILE: Option<BufWriter<File>> = None;

fn main() {
    // let mut screen_buff = [0u16; IMG_WIDTH * IMG_HEIGHT];
    // let mut rgb_buff = [0u8; IMG_WIDTH * IMG_HEIGHT * 3];

    unsafe {
        LOG_FILE = Some(BufWriter::new(File::create("log1.txt.tns").unwrap()));
    }

    let mut screen_buff = vec![0u16; IMG_WIDTH * IMG_HEIGHT];
    let mut rgb_buff = vec![0u8; IMG_WIDTH * IMG_HEIGHT * 3];

    let sample_count =
        ndless::msg::msg_numeric("Sample Input", "Weeeeeeeeee", "How many samples?", (1, 100));
    if sample_count.is_none() {
        return;
    }
    let sample_count = sample_count.unwrap();

    // let sample_count = 1;

    let camera = Camera::new(
        Vec3FI32::new(fxi32!(0), fxi32!(0), fxi32!(0)),
        Vec3FI32::new(fxi32!(0), fxi32!(0), fxi32!(-1)),
        Vec3FI32::new(fxi32!(0), fxi32!(1), fxi32!(0)),
        fxi32!(45),
        fxi32!(IMG_WIDTH as i32) / fxi32!(IMG_HEIGHT as i32),
    );

    let ground_material = Rc::new(Lambertian::new(Vec3FI32::new(
        fxi32!(0.2),
        fxi32!(0.95),
        fxi32!(0.2),
    )));

    let basic_material = Rc::new(Lambertian::new(Vec3FI32::new(
        fxi32!(0.95),
        fxi32!(0.2),
        fxi32!(0.2),
    )));

    let cooler_material = Rc::new(Metal::new(
        fxi32!(0.1),
        Vec3FI32::new(fxi32!(0.7), fxi32!(0.7), fxi32!(0.7)),
    ));

    let world_vec: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Vec3FI32::new(fxi32!(0), fxi32!(-101.5), fxi32!(0)),
            fxi32!(100.0),
            Some(ground_material),
        )),
        Box::new(Sphere::new(
            Vec3FI32::new(fxi32!(2.0), fxi32!(0), fxi32!(-2)),
            fxi32!(1.35),
            Some(cooler_material),
        )),
        Box::new(Sphere::new(
            Vec3FI32::new(fxi32!(-0.5), fxi32!(0), fxi32!(-2)),
            fxi32!(1),
            Some(basic_material),
        )),
    ];

    let renderer = Renderer::new(
        camera,
        HittableList::new(world_vec),
        IMG_WIDTH as u16,
        IMG_HEIGHT as u16,
        sample_count as u16,
    );

    init_screen();

    renderer.render_scene(&mut screen_buff, &mut rgb_buff, &mut |buffer, _| {
        blit_buffer(buffer);
    });

    dither::dither(&rgb_buff, &mut screen_buff);

    blit_buffer(&mut screen_buff);

    wait_key_pressed();

    deinit_screen();

    unsafe {
        LOG_FILE = None;
    }
}
