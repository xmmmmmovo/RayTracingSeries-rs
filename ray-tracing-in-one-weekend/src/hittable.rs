use std::sync::Arc;

use crate::{
    material::Scatter,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Scatter>,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;

        let mut closest_so_far = t_max;

        self.iter().for_each(|object| {
            if let Some(h_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = h_rec.t;
                temp_rec = Some(h_rec);
            }
        });

        temp_rec
    }
}
