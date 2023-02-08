use super::vector::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn ray_intersect(&self, origin: Vec3, ray: Vec3) -> Option<Vec3> {
        // Computes the position vector of the first intersection between the ray and the sphere, if any

        // Position vector of sphere from the provided origin
        let center = self.center - origin;

        if ray.dot(center) < 0f32 {
            // Sphere is behind the ray
            return None;
        }

        if center.sq_magnitude() < self.radius * self.radius {
            // Origin is within the sphere
            return None;
        }

        // Position vector of the closest point on the ray to the sphere center
        let closest = ray * (ray.dot(center) / ray.magnitude());

        let center_closest = closest - center;

        if center_closest.sq_magnitude() > self.radius * self.radius {
            // No intersection
            return None;
        }

        let intersection = closest
            - ray * (self.radius * self.radius - center_closest.sq_magnitude()) / ray.magnitude();

        Some(intersection)
    }
}
