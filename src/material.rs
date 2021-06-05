use super::{color::Color3d, hit::Hit, ray::Ray, vec::Vec3d};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &Hit, attenuation: &mut Color3d, scattered: &mut Ray);
}

pub struct Lambertian {
    pub albedo: Color3d,
}

impl Lambertian {
    pub const fn new(albedo: Color3d) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &Hit, attenuation: &mut Color3d, scattered: &mut Ray) {
        let mut scatter_direction = rec.normal + Vec3d::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;
    }
}

pub struct Metal {
    pub albedo: Color3d,
    fuzziness: f32,
}

impl Metal {
    pub const fn new(albedo: Color3d, mut fuzziness: f32) -> Metal {
        // cast float to integer to work around const float arithmatic not being stable
        if fuzziness as usize > 1 {
            fuzziness = 1.;
        }
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &Hit, attenuation: &mut Color3d, scattered: &mut Ray) {
        let reflected = r_in.direction.unit_vector().reflect(&rec.normal)
            + self.fuzziness * Vec3d::random_in_unit_sphere();
        *scattered = Ray::new(rec.point, reflected);
        *attenuation = self.albedo;

        // If we aren't scattering somewhat off the material, kill the ray
        if scattered.direction.dot(&rec.normal) <= 0. {
            *attenuation = Color3d::new(0., 0., 0.);
        }
    }
}
