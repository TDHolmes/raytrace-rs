use super::{material::Material, point::Point3d, ray::Ray, vec::Vec3d};

pub struct Hit {
    pub point: Point3d,
    pub normal: Vec3d,
    pub time: f32,
    pub front_face: bool,
    pub material: &'static dyn Material,
}

impl Hit {
    pub fn new(ray: &Ray, time: f32, normal: &Vec3d, material: &'static dyn Material) -> Hit {
        let point = ray.at(time);
        let mut hit = Hit {
            time,
            point,
            normal: *normal,
            front_face: false,
            material,
        };
        hit.set_face_normal(ray);

        hit
    }

    fn set_face_normal(&mut self, ray: &Ray) {
        self.front_face = ray.direction.dot(&self.normal) < 0.;
        if self.front_face {
            self.normal = self.normal;
        } else {
            self.normal = -self.normal;
        }
    }
}

impl std::fmt::Debug for Hit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hit {:?} @ {:?}", self.point, self.time)?;
        Ok(())
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<Hit>;
}

pub trait PrintableHittable: Hittable + std::fmt::Debug {}

pub struct HitList {
    hittable_objects: Vec<Box<dyn PrintableHittable>>,
}

impl HitList {
    pub fn new() -> HitList {
        HitList {
            hittable_objects: vec![],
        }
    }

    pub fn add(&mut self, new_obj: Box<impl PrintableHittable + 'static>) {
        self.hittable_objects.push(new_obj);
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<Hit> {
        // let mut closest_hit_time: f32 = time_max;
        let mut hit: Option<Hit> = None;

        for object in &self.hittable_objects {
            if let Some(this_hit) = object.hit(ray, time_min, time_max) {
                hit = Some(this_hit);
                // println!("\t{:?} is hittable @ {:?}", object, hit);
            }
        }

        hit
    }
}
