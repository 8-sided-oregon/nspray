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
    fn scatter(
        &self,
        rand: &mut Rand32,
        _ray: &Ray,
        record: &HitRecord,
    ) -> Option<(Ray, Vec3FI32)> {
        let mut scattered_dir = record.normal + Vec3FI32::random_in_unit_sphere(rand);

        if scattered_dir.near_zero() {
            scattered_dir = record.normal;
        }

        Some((Ray::new(record.point, scattered_dir), self.albedo))
    }
}

pub struct CheckeredLambertian {
    albedo1: Vec3FI32,
    albedo2: Vec3FI32,
}

impl CheckeredLambertian {
    pub fn new(albedo1: Vec3FI32, albedo2: Vec3FI32) -> Self {
        Self { albedo1, albedo2 }
    }
}

impl Material for CheckeredLambertian {
    fn scatter(
        &self,
        rand: &mut Rand32,
        _ray: &Ray,
        record: &HitRecord,
    ) -> Option<(Ray, Vec3FI32)> {
        let mut scattered_dir = record.normal + Vec3FI32::random_in_unit_sphere(rand);

        if scattered_dir.near_zero() {
            scattered_dir = record.normal;
        }

        let mut albedo = self.albedo1;

        if let Some(ref mapped) = record.mapped_point {
            let x = mapped.x.modulo(fxi32!(2)).abs();
            let y = mapped.y.modulo(fxi32!(2)).abs();

            if (x > fxi32!(1) && y < fxi32!(1)) || (x < fxi32!(1) && y > fxi32!(1)) {
                albedo = self.albedo2;
            }
        }

        Some((Ray::new(record.point, scattered_dir), albedo))
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
