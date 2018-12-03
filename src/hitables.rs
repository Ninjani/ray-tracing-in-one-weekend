use materials::Material;
use rand::Rng;
use rays::Ray;
use std::f64::MAX;
use vectors::Vec3;
use std::sync::Arc;

pub fn color_world<R: Rng>(ray: &Ray, world: &[Sphere], depth: i32, rng: &mut R) -> Vec3 {
    match hit(world, ray, 0.001, MAX) {
        Some(record) => match record.material.scatter(ray, &record, rng) {
            Some((attenuation, scattered)) => {
                if depth < 50 {
                    color_world(&scattered, world, depth + 1, rng) * attenuation
                } else {
                    Vec3::empty()
                }
            }
            None => Vec3::empty(),
        },
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.);
            Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
        }
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                    material: Arc::clone(&self.material),
                });
            }
        }
        None
    }
}

pub fn hit(spheres: &[Sphere], ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut temp_record: Option<HitRecord> = None;
    for sphere in spheres.iter() {
        if let Some(record) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                temp_record = Some(record);
        }
    }
    temp_record
}
