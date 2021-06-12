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

pub struct Dialectric {
    refraction_index: f32,
}

impl Dialectric {
    pub const fn new(refraction_index: f32) -> Dialectric {
        Dialectric { refraction_index }
    }

    fn refract(uv: &Vec3d, n: &Vec3d, etai_over_etat: f32) -> Vec3d {
        let cos_theta = f32::min((-*uv).dot(n), 1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * *n;
        return r_out_perp + r_out_parallel;
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1. - r0) * (1. - cosine).powf(5.);
    }
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &Hit, attenuation: &mut Color3d, scattered: &mut Ray) {
        *attenuation = Color3d::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = f32::min((-unit_direction).dot(&rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dialectric::reflectance(cos_theta, refraction_ratio) > fastrand::f32()
        {
            unit_direction.reflect(&rec.normal)
        } else {
            Dialectric::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.point, direction);
    }
}
