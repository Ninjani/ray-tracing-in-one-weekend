use hitables::HitRecord;
use rand::Rng;
use rays::Ray;
use vectors::Vec3;

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ref_idx: f64 },
}

fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2. - 1.;
        if p.dot(p) < 1. {
            return p;
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Material {
    pub fn lambertian(albedo: Vec3) -> Material {
        Material::Lambertian { albedo }
    }
    pub fn metal(albedo: Vec3, fuzz: f64) -> Material {
        if fuzz > 1. {
            return Material::Metal { albedo, fuzz: 1. };
        }
        Material::Metal { albedo, fuzz }
    }
    pub fn dielectric(ref_idx: f64) -> Material {
        Material::Dielectric { ref_idx }
    }
    pub fn scatter<R: Rng>(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        rng: &mut R,
    ) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let target = record.p + record.normal + random_in_unit_sphere(rng);
                Some((*albedo, Ray::new(record.p, target - record.p)))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(ray_in.direction.unit_vector(), record.normal);
                let scattered = Ray::new(record.p, reflected + random_in_unit_sphere(rng) * *fuzz);
                if scattered.direction.dot(record.normal) > 0. {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
            Material::Dielectric { ref_idx } => {
                let reflected = reflect(ray_in.direction, record.normal);
                let (outward_normal, ni_over_nt, cosine) = if ray_in.direction.dot(record.normal)
                    > 0.
                {
                    let mut cosine = ray_in.direction.dot(record.normal) / ray_in.direction.length();
                    cosine = (1. - ref_idx * ref_idx * (1. - cosine * cosine)).sqrt();
                    (-record.normal, *ref_idx, cosine)
                } else {
                    let cosine = -ray_in.direction.dot(record.normal) / ray_in.direction.length();
                    (record.normal, 1. / ref_idx, cosine)
                };
                let attenuation = Vec3::new(1., 1., 1.);
                match refract(ray_in.direction, outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        let reflect_prob = schlick(cosine, *ref_idx);
                        if rng.gen::<f64>() < reflect_prob {
                            Some((attenuation, Ray::new(record.p, reflected)))
                        } else {
                            Some((attenuation, Ray::new(record.p, refracted)))
                        }
                    }
                    None => Some((attenuation, Ray::new(record.p, reflected))),
                }
            }
        }
    }
}
