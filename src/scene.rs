use super::geometry::{sphere::Sphere, vector::Vec3};
use std::vec::Vec;

#[derive(Copy, Clone, Debug, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl IntoIterator for &Pixel {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.r, self.g, self.b].into_iter()
    }
}

#[derive(Debug)]
pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub fov: f32, // degrees
    pub spheres: Vec<Sphere>,
    pub buffer: Vec<Pixel>,
}

impl Scene {
    pub fn new(width: usize, height: usize, fov: f32) -> Scene {
        Scene {
            width,
            height,
            fov,
            spheres: Vec::new(),
            buffer: vec![Pixel::default(); width * height],
        }
    }

    pub fn render(&mut self) {
        let camera_pos = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let fov_xz = self.fov * std::f32::consts::PI * 2.0 / 360.0;
        let fov_yz = fov_xz * (self.height as f32) / (self.width as f32);

        for i in (0..self.height) {
            for j in (0..self.width) {
                let pixel_pos = Vec3 {
                    x: (j as f32 * (fov_xz / self.width as f32) - fov_xz / 2.0).tan(),
                    y: (fov_yz / 2.0 - i as f32 * (fov_yz / self.height as f32)).tan(),
                    z: -1.0,
                };

                let ray = pixel_pos - camera_pos;
                let pixel = if self
                    .spheres
                    .iter()
                    .any(|s| s.ray_intersect(camera_pos, ray).is_some())
                {
                    Pixel {
                        r: 128,
                        g: 128,
                        b: 0,
                    }
                } else {
                    Pixel {
                        r: 0,
                        g: 128,
                        b: 128,
                    }
                };

                self.buffer[i * self.width + j] = pixel;
            }
        }

        // let width = self.width;
        // let height = self.height;

        // self.buffer = (0..height)
        //     .flat_map(|i| {
        //         (0..width).map(move |j| Pixel {
        //             r: (i * 255 / height) as u8,
        //             g: (j * 255 / width) as u8,
        //             b: 0,
        //         })
        //     })
        //     .collect();
    }
}
