use oorandom::Rand32;

use crate::{
    fixed::{self, FixedI32},
    fxi32,
    ray::Ray,
    vec3::Vec3FI32,
};

pub struct Camera {
    origin: Vec3FI32,
    top_left: Vec3FI32,
    horizontal: Vec3FI32,
    vertical: Vec3FI32,
    lens_radius: FixedI32,
    u: Vec3FI32,
    v: Vec3FI32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3FI32,
        lookat: Vec3FI32,
        vup: Vec3FI32,
        vfov: FixedI32,
        aspect_ratio: FixedI32,
        apeture: FixedI32,
        focus_dist: FixedI32,
    ) -> Self {
        let focal_length = FixedI32::from(1);
        let viewport_height = focal_length * 2 * (vfov * fixed::PI / 180).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;

        let top_left = origin - horizontal / fxi32!(2) + vertical / fxi32!(2) - w * focus_dist;
        let lens_radius = apeture / 2;

        Self {
            origin,
            top_left,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray_noblur(&self, u: FixedI32, v: FixedI32) -> Ray {
        Ray::new(
            self.origin,
            self.top_left + self.horizontal * u - self.vertical * v - self.origin,
        )
    }

    pub fn get_ray_blur(&self, rand: &mut Rand32, s: FixedI32, t: FixedI32) -> Ray {
        let rd = Vec3FI32::random_in_unit_disk(rand) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.top_left + self.horizontal * s - self.vertical * t - self.origin - offset,
        )
    }
}
