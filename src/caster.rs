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
                    .get_ray(step_u * (j as i32), step_v * (i as i32));

                let color = self.ray_color(&ray);

                let r: i32 = (color.x * 0x1f).into();
                let g: i32 = (color.y * 0x1f).into();
                let b: i32 = (color.z * 0x1f).into();

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

    fn ray_color(&self, ray: &Ray) -> Vec3FI32 {
        let unit_dir = ray.dir().unit_vector();
        let t = (unit_dir.y + 1) * fxi32!(0.5);

        Vec3FI32::from(fxi32!(1)) * (fxi32!(1) - t)
            + Vec3FI32::new(fxi32!("0.5"), fxi32!("0.7"), fxi32!("1")) * t
    }
}
