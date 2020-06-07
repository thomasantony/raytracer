use image::ImageBuffer;
use raytracer::{color, point3, Color, Point3, Ray, Vec3};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    // No intersection
    if discriminant <= 0. {
        return None;
    } else {
        // This is distance to the sphere along the ray for near side point
        let position = (-half_b - discriminant.sqrt()) / (2.0 * a);
        return Some(position);
    }
}
fn ray_color(r: &Ray) -> Color {
    if let Some(t) = hit_sphere(&point3(0., 0., -1.), 0.5, r) {
        // Make sure sphere is in front of the ray
        if t > 0. {
            // Normal unit vector has components from -1 to 1
            // Map those components to R, G, B
            let n = (r.at(t) - Vec3::new(0., 0., -1.)).unit();
            return 0.5 * color(n.x() + 1., n.y() + 1., n.z() + 1.);
        }
    }
    let unit_direction = r.direction.unit();
    // Convert y-component (-1 to 1) to blue color
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio as f32) as u32;

    let mut im = ImageBuffer::new(image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            let pixel = image::Rgb(pixel_color.to_rgb());
            im.put_pixel(i, image_height - j - 1, pixel);
        }
    }
    println!("");
    im.save("./04_output.png").unwrap();
}
