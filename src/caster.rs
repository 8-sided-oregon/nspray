use oorandom::Rand32;

use crate::{
    camera::Camera,
    fixed::FixedI32,
    fxi32,
    hittable::{HitRecord, Hittable, HittableList},
    ray::Ray,
    vec3::Vec3FI32,
};

pub struct Renderer {
    camera: Camera,
    scene: HittableList,
    width: u16,
    height: u16,
    samples: u16,
}

impl Renderer {
    pub fn new(camera: Camera, scene: HittableList, width: u16, height: u16, samples: u16) -> Self {
        Self {
            camera,
            scene,
            width,
            height,
            samples,
        }
    }

    pub fn render_scene<F>(&self, buffer: &mut [u16], progress_callback: &mut F)
    where
        F: FnMut(&mut [u16], u16),
    {
        let mut rand = Rand32::new(1);

        let expected = (self.width as usize) * (self.height as usize);
        assert_eq!(
            buffer.len(),
            expected,
            "Image buffer is not the correct size."
        );

        let step_v = fxi32!(1) / (self.height as i32);
        let step_u = fxi32!(1) / (self.width as i32);

        for i in 0..self.height {
            for j in 0..self.width {
                let ray = self
                    .camera
                    .get_ray(step_u * (j as i32), step_v * (i as i32));

                let mut color = Vec3FI32::default();
                for _ in 0..self.samples {
                    let new_ray = Ray::new(
                        ray.origin(),
                        ray.dir()
                            + Vec3FI32::new(
                                FixedI32::rand(&mut rand) / 300,
                                FixedI32::rand(&mut rand) / 300,
                                FixedI32::rand(&mut rand) / 300,
                            ),
                    );
                    color += self.ray_color(&new_ray, &self.scene, &mut rand, 10);
                }
                color = color / fxi32!(self.samples);

                let r: i32 = (color.x.sqrt() * 0x1f).into();
                let g: i32 = (color.y.sqrt() * 0x1f).into();
                let b: i32 = (color.z.sqrt() * 0x1f).into();

                let r = r.clamp(0, 0x1f) as u16;
                let g = g.clamp(0, 0x1f) as u16;
                let b = b.clamp(0, 0x1f) as u16;

                let index = (i as usize) * (self.width as usize) + (j as usize);

                // RGB555
                buffer[index] = r << 10 | g << 5 | b;
            }

            if i % 10 == 0 {
                progress_callback(buffer, i);
            }
        }
    }

    fn ray_color(
        &self,
        ray: &Ray,
        world: &impl Hittable,
        rand: &mut Rand32,
        ray_num: u8,
    ) -> Vec3FI32 {
        if ray_num == 0 {
            return Vec3FI32::default();
        }

        let mut rec = HitRecord::default();

        if world.hit(ray, &mut rec, fxi32!(0.001), fxi32!(1000)) {
            if let Some(ref material) = rec.material {
                if let Some((new_ray, attenuation)) = material.scatter(rand, ray, &rec) {
                    return self.ray_color(&new_ray, world, rand, ray_num - 1) * attenuation;
                }
            }

            return Vec3FI32::default();
        }

        let unit_dir = ray.dir().unit_vector();
        let t = (unit_dir.y + 1) * fxi32!(0.5);

        Vec3FI32::from(fxi32!(1)) * (fxi32!(1) - t)
            + Vec3FI32::new(fxi32!("0.5"), fxi32!("0.7"), fxi32!("1")) * t
    }
}
