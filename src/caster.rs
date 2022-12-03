use alloc::format;

use crate::{camera::Camera, fxi32, ray::Ray, vec3::Vec3FI32, world::World};

pub struct Renderer {
    camera: Camera,
    scene: World,
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new(camera: Camera, scene: World, width: u16, height: u16) -> Self {
        Self {
            camera,
            scene,
            width,
            height,
        }
    }

    pub fn render_scene<F>(&self, buffer: &mut [u16], progress_callback: &mut F)
    where
        F: FnMut(&mut [u16], u16),
    {
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
                    .get_ray(step_v * (i as i32), step_u * (j as i32));
                let color = self.ray_color(&ray);

                let r: i32 = (color.x * 255).into();
                let g: i32 = (color.y * 255).into();
                let b: i32 = (color.z * 255).into();

                let index = (i as usize) * (self.width as usize) + (j as usize);

                // RGB555
                buffer[index] =
                    (r as u16 & 0x1f) << 10 | (g as u16 & 0x1f) << 5 | (b as u16 & 0x1f);
            }

            if i % 10 == 9 {
                progress_callback(buffer, i);
            }
        }
    }

    fn ray_color(&self, ray: &Ray) -> Vec3FI32 {
        let unit_dir = ray.dir().unit_vector();
        let t = (unit_dir.y + 1) * fxi32!(0.5);

        Vec3FI32::from(fxi32!(1)) * (fxi32!(1) - t)
            + Vec3FI32::new(fxi32!("0.5"), fxi32!("0.7"), fxi32!("1")) * t
    }
}
