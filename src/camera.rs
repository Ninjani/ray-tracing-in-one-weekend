use rand::Rng;
use rays::Ray;
use std::f64::consts::PI;
use vectors::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

fn random_in_unit_disk<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.) * 2. - Vec3::new(1., 1., 0.);
        if p.dot(p) < 1. {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.;
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);
        let lower_left_corner =
            origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * 2. * half_width * focus_dist;
        let vertical = v * 2. * half_height * focus_dist;
        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray<R: Rng>(&self, s: f64, t: f64, rng: &mut R) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
