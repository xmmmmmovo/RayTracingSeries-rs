use crate::{
    ray::Ray,
    vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,

    cu: Vec3,
    cv: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        // 光圈大小
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let viewport_height = (vfov.to_radians() / 2.0).tan() * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        // Camera coordinate system
        let cw = unit_vector(&(lookfrom - lookat));
        let cu = unit_vector(&cross(&vup, &cw));
        let cv = cross(&cw, &cu);

        let h = cu * viewport_width * focus_dist;
        let v = cv * viewport_height * focus_dist;

        let llc = lookfrom - h / 2.0 - v / 2.0 - cw * focus_dist;

        Camera {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
            cu,
            cv,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.cu * rd.x + self.cv * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
