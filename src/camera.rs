use crate::{Point3, Ray, Vec3};
use crate::utils::random_in_unit_disk;

pub struct Camera {
    origin: Point3,
    u: Vec3,
    v: Vec3, 
    w: Vec3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn default() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            lens_radius: 0.,
        }
    }
    pub fn new(look_from: Point3,
               look_at: Point3,
               vup: Vec3,
               vfov_deg: f64, 
               aspect_ratio: f64) -> Self {
        let vfov = vfov_deg.to_radians();
        let h = (vfov/2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: 0.,
        }
    }
    pub fn new_with_depth_of_field(look_from: Point3,
               look_at: Point3,
               vup: Vec3,
               vfov_deg: f64, 
               aspect_ratio: f64,
            aperture: f64,
            focus_dist: f64) -> Self 
    {
        let vfov = vfov_deg.to_radians();
        let h = (vfov/2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - focus_dist*w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.
        }
    }
    // The arguments are called u and v in initial sections of the book
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // Apply depth-of-field if needed
        if self.lens_radius > 0.
        {
            let rd = self.lens_radius * random_in_unit_disk();
            let offset = self.u * rd.x() + self.v * rd.y();
            Ray::new(
                self.origin + offset,
                self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            )
        }else{
            Ray::new(
                self.origin,
                self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
            )
        }
    }
}
