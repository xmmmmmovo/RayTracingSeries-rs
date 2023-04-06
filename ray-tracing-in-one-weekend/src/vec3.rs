use std::ops::Range;

use glam::DVec3;
use rand::Rng;

pub type Vec3 = DVec3;
pub type Point3 = DVec3;
pub type Color = DVec3;

pub fn random(range: Range<f64>) -> Vec3 {
    Vec3::new(
        rand::thread_rng().gen_range(range.clone()),
        rand::thread_rng().gen_range(range.clone()),
        rand::thread_rng().gen_range(range),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random(-1.0..1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn near_zero(v: Vec3) -> bool {
    v.x.abs() < f64::EPSILON && v.y.abs() < f64::EPSILON && v.z.abs() < f64::EPSILON
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

pub fn refract(v: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-v).dot(n).min(1.0);
    let r_out_perp = (v + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perp.length_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

pub fn convert_color(color: &Color, samples_per_pixel: u32) -> u32 {
    let r = (256.0
        * (color.x / (samples_per_pixel as f64))
            .sqrt()
            .clamp(0.0, 0.999)) as u32;
    let g = (256.0
        * (color.y / (samples_per_pixel as f64))
            .sqrt()
            .clamp(0.0, 0.999)) as u32;
    let b = (256.0
        * (color.z / (samples_per_pixel as f64))
            .sqrt()
            .clamp(0.0, 0.999)) as u32;
    (r << 16) | (g << 8) | b
}
