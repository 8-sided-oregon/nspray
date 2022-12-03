//#![cfg_attr(not(test), no_std)]

#![no_std]
#![feature(format_args_capture)]

//#![cfg_attr(not(feature = "std"), no_std)]

use core::time::Duration;

use alloc::format;
use camera::Camera;
use caster::Renderer;
use ndless::{
    input::{self, wait_key_pressed},
    thread,
};
use screen::{blit_buffer, deinit_screen, init_screen};
use vec3::Vec3FI32;
use world::World;

extern crate ndless;
extern crate ndless_sys;

#[cfg(not(test))]
extern crate ndless_handler;

extern crate alloc;

mod camera;
mod caster;
mod debug;
mod fixed;
mod hittable;
mod ray;
mod screen;
mod tests;
mod vec3;
mod world;

fn main() {
    init_screen();

    let mut screen_buff = [0u16; 320 * 240];

    let camera = Camera::new(
        Vec3FI32::new(fxi32!(-2), fxi32!(2), fxi32!(1)),
        Vec3FI32::new(fxi32!(0), fxi32!(0), fxi32!(-1)),
        Vec3FI32::new(fxi32!(0), fxi32!(1), fxi32!(0)),
        fxi32!(90),
        fxi32!(320) / fxi32!(240),
    );

    let renderer = Renderer::new(camera, World {}, 320, 240);

    renderer.render_scene(&mut screen_buff, &mut |buffer, line| {
        blit_buffer(buffer);
        //ndless::msg::msg("Hey", &format!("{line}"));
    });

    blit_buffer(&mut screen_buff);

    wait_key_pressed();

    deinit_screen();
}