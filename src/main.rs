mod geometry;
mod scene;

use scene::Scene;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut scene = Scene::new(1024, 768, 75.0);
    scene.spheres.push(geometry::sphere::Sphere {
        center: geometry::vector::Vec3 {
            x: 0.5,
            y: 0.0,
            z: -3.0,
        },
        radius: 0.75,
    });
    scene.spheres.push(geometry::sphere::Sphere {
        center: geometry::vector::Vec3 {
            x: -1.0,
            y: 1.0,
            z: -2.0,
        },
        radius: 0.5,
    });
    scene.spheres.push(geometry::sphere::Sphere {
        center: geometry::vector::Vec3 {
            x: 1.0,
            y: -1.0,
            z: -2.0,
        },
        radius: 0.25,
    });

    scene.render();

    let mut file = std::fs::File::create("out.ppm")?;
    writeln!(file, "P6 {} {} 255", scene.width, scene.height)?;
    file.write_all(&scene.buffer.iter().flatten().collect::<Vec<u8>>())?;

    Ok(())
}
