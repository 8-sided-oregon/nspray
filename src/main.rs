//#![cfg_attr(not(test), no_std)]

#![no_std]
#![feature(format_args_capture)]

//#![cfg_attr(not(feature = "std"), no_std)]

use alloc::{
    boxed::Box,
    format,
    rc::Rc,
    vec::{self, Vec},
};
use camera::Camera;
use caster::Renderer;
use hittable::{Hittable, HittableList, Sphere};
use material::{Lambertian, Metal};
use ndless::input::wait_key_pressed;
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
mod fixed;
mod hittable;
mod material;
mod ray;
mod screen;
mod tests;
mod vec3;

fn main() {
    let mut screen_buff = [0u16; 320 * 240];

    let sample_count =
        ndless::msg::msg_numeric("Sample Input", "Weeeeeeeeee", "How many samples?", (1, 100));
    if sample_count.is_none() {
        return;
    }
    let sample_count = sample_count.unwrap();

    let camera = Camera::new(
        Vec3FI32::new(fxi32!(0), fxi32!(0), fxi32!(0)),
        Vec3FI32::new(fxi32!(0), fxi32!(0), fxi32!(-1)),
        Vec3FI32::new(fxi32!(0), fxi32!(1), fxi32!(0)),
        fxi32!(45),
        fxi32!(320) / fxi32!(240),
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
        fxi32!(0.2),
        Vec3FI32::new(fxi32!(0.7), fxi32!(0.7), fxi32!(0.7)),
    ));

    let mut world_vec: Vec<Box<dyn Hittable>> = Vec::new();

    world_vec.push(Box::new(Sphere::new(
        Vec3FI32::new(fxi32!(0), fxi32!(-101.5), fxi32!(0)),
        fxi32!(100.0),
        Some(ground_material.clone()),
    )));

    world_vec.push(Box::new(Sphere::new(
        Vec3FI32::new(fxi32!(0.5), fxi32!(0), fxi32!(-2)),
        fxi32!(0.5),
        Some(cooler_material),
    )));

    world_vec.push(Box::new(Sphere::new(
        Vec3FI32::new(fxi32!(-0.5), fxi32!(0), fxi32!(-2)),
        fxi32!(1),
        Some(basic_material),
    )));

    let renderer = Renderer::new(
        camera,
        HittableList::new(world_vec),
        320,
        240,
        sample_count as u16,
    );

    init_screen();

    renderer.render_scene(&mut screen_buff, &mut |buffer, _| {
        blit_buffer(buffer);
    });

    blit_buffer(&mut screen_buff);

    wait_key_pressed();

    deinit_screen();
}
