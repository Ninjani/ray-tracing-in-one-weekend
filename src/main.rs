#![feature(crate_in_paths)]
extern crate rand;

pub mod camera;
pub mod hitables;
pub mod materials;
pub mod rays;
pub mod vectors;

use crate::vectors::Vec3;
use crate::camera::Camera;
use crate::hitables::{color_world, Sphere};
use crate::materials::Material;

use rand::Rng;
use std::sync::Arc;

fn random_scene<R: Rng>(rng: &mut R) -> Vec<Sphere> {
    let n = 500;
    let mut world = Vec::with_capacity(n + 1);
    world.push(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Arc::new(Material::lambertian(Vec3::new(0.5, 0.5, 0.5)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Vec3::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_material < 0.8 {
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Material::lambertian(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    ));
                } else if choose_material < 0.95 {
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Material::metal(
                            Vec3::new(
                                0.5 * (1. + rng.gen::<f64>()),
                                0.5 * (1. + rng.gen::<f64>()),
                                0.5 * (1. + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        )),
                    ));
                } else {
                    world.push(Sphere::new(center, 0.2, Arc::new(Material::dielectric(1.5))));
                }
            }
        }
    }
    world.push(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Arc::new(Material::dielectric(1.5)),
    ));
    world.push(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Arc::new(Material::lambertian(Vec3::new(0.4, 0.2, 0.1))),
    ));
    world.push(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Arc::new(Material::metal(Vec3::new(0.7, 0.6, 0.5), 0.)),
    ));
    world
}

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 500;
    let mut rng = rand::thread_rng();
    print!("P3\n{}\n{}\n255\n", nx, ny);
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0., 1., 0.),
        20.,
        f64::from(nx) / f64::from(ny),
        aperture,
        dist_to_focus,
    );
    let world = random_scene(&mut rng);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::empty();
            for _ in 0..ns {
                let s = (f64::from(i) + rng.gen::<f64>()) / f64::from(nx);
                let t = (f64::from(j) + rng.gen::<f64>()) / f64::from(ny);
                let ray = cam.get_ray(s, t, &mut rng);
                color += color_world(&ray, &world, 0, &mut rng);
            }
            color /= f64::from(ns);
            color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
            color *= 255.99;
            print!("{} {} {}\n", color.x as i64, color.y as i64, color.z as i64);
        }
    }
}
