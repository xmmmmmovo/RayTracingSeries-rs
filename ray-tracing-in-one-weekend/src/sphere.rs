use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    hittable::{HitRecord, Hittable},
    material::Scatter,
    ray::Ray,
    vec3::{dot, Point3},
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Scatter>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        // 解方程得根
        let mut root = (-half_b - sqrtd) / a;
        if f64::EPSILON < (t_min - root) || t_max < root {
            root = (-half_b + sqrtd) / a;
            if f64::EPSILON < (t_min - root) || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            p: r.at(root),
            normal: Point3::new(0.0, 0.0, 0.0),
            t: root,
            front_face: false,
            material: self.material.clone(),
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
