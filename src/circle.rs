use crate::{
    hit::{Hit, Hittable, PrintableHittable},
    material::Material,
    point::Point3d,
    ray::Ray,
};

pub struct Circle {
    pub radius: f32,
    pub origin: Point3d,
    material: &'static dyn Material,
}

impl Circle {
    pub fn new(radius: f32, origin: Point3d, material: &'static dyn Material) -> Circle {
        Circle {
            radius,
            origin,
            material,
        }
    }
}

impl Hittable for Circle {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;

        if discriminant >= 0. {
            let root = (-b - discriminant.sqrt()) / (2. * a);
            if root >= time_min && root <= time_max {
                let hit_point = ray.at(root);
                let normal = (hit_point - self.origin) / self.radius;
                return Some(Hit::new(&ray, root, &normal, self.material));
            }
        }

        None
    }
}

impl PrintableHittable for Circle {}

impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle({:0.3}, {:?})", self.radius, self.origin)?;

        Ok(())
    }
}
