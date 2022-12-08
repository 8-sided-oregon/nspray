use oorandom::Rand32;

use crate::{fixed::FixedI32, fxi32, hittable::HitRecord, ray::Ray, vec3::Vec3FI32};

fn reflect(v: Vec3FI32, n: Vec3FI32) -> Vec3FI32 {
    v - n * v.dot(n) * fxi32!(2)
}

pub trait Material {
    fn scatter(&self, rand: &mut Rand32, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3FI32)>;
}

pub struct Lambertian {
    albedo: Vec3FI32,
}

impl Lambertian {
    pub fn new(albedo: Vec3FI32) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rand: &mut Rand32, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3FI32)> {
        let mut scattered_dir = record.normal + Vec3FI32::random_in_unit_sphere(rand);

        if scattered_dir.near_zero() {
            scattered_dir = record.normal;
        }

        Some((Ray::new(record.point, scattered_dir), self.albedo))
    }
}

pub struct Metal {
    fuzziness: FixedI32,
    albedo: Vec3FI32,
}

impl Metal {
    pub fn new(fuzziness: FixedI32, albedo: Vec3FI32) -> Self {
        Self { fuzziness, albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, rand: &mut Rand32, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3FI32)> {
        let noise = Vec3FI32::random_in_unit_sphere(rand) * self.fuzziness;

        let reflected = reflect(ray.dir().unit_vector(), record.normal) + noise;
        let new_ray = Ray::new(record.point, reflected);

        if new_ray.dir().dot(record.normal) > fxi32!(0) {
            Some((new_ray, self.albedo))
        } else {
            None
        }
    }
}
