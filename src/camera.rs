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
}

impl Camera {
    pub fn new(
        lookfrom: Vec3FI32,
        lookat: Vec3FI32,
        vup: Vec3FI32,
        vfov: FixedI32,
        aspect_ratio: FixedI32,
    ) -> Self {
        let focal_length = FixedI32::from(1);
        let viewport_height = focal_length * 2 * (vfov * fixed::PI / 180).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        let top_left = origin - horizontal / fxi32!(2) + vertical / fxi32!(2) - w;

        Self {
            origin,
            top_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: FixedI32, v: FixedI32) -> Ray {
        Ray::new(
            self.origin,
            self.top_left + self.horizontal * u - self.vertical * v - self.origin,
        )
    }
}
