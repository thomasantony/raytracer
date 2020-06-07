use image::ImageBuffer;
use rand::prelude::*;
use raytracer::{color, point3, Camera, Color, Hittable, HittableList, Ray, SimpleSphere,
                rand_unit_vector};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0
    {
        return color(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) 
    {
        let target = rec.point + rec.normal + rand_unit_vector();
        let random_ray = Ray::new(rec.point, target - rec.point);
        // Cut intensity in half with every reflection
        return 0.5 * ray_color(&random_ray, world, depth-1);
    }
    let unit_direction = r.direction.unit();
    // Convert y-component (-1 to 1) to blue color
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
fn main() {
    let mut rng = thread_rng();

    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio as f32) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut im = ImageBuffer::new(image_width, image_height);

    let camera = Camera::new();
    let mut world = HittableList::new();
    world.add(SimpleSphere::new(point3(0., 0., -1.), 0.5));
    world.add(SimpleSphere::new(point3(0., -100.5, -1.), 100.));

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = color(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = ((j as f64) + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            let pixel = image::Rgb(pixel_color.to_rgb_scaled_gamma2(samples_per_pixel));
            im.put_pixel(i, image_height - j - 1, pixel);
        }
    }
    println!("");
    im.save("./08_output.png").unwrap();
}
