use std::rc::Rc;
use image::ImageBuffer;
use rand::prelude::*;
use raytracer::{color, point3, Vec3, Camera, Color, Hittable, HittableList, Ray, Sphere};
use raytracer::materials::{Material, Lambertian, Metal, Dielectric};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0
    {
        return color(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) 
    {
        if let Some((scattered_ray, attenuation)) = rec.scatter(r)
        {
            return attenuation * ray_color(&scattered_ray, world, depth -1);
        }
        return color(0., 0., 0.);
    }
    let unit_direction = r.direction.unit();
    // Convert y-component (-1 to 1) to blue color
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
fn random_scene() -> HittableList {
    let mut rng = thread_rng();
    let mut world = HittableList::new();
    
    let ground_material = Rc::new(Lambertian::new(color(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(point3(0., -1e3, 0.), 1000., ground_material.clone())));

    for a in -11 .. 11
    {
        for b in -11 .. 11
        {
            let choose_mat = rng.gen::<f64>();
            let center = point3(a as f64 + 0.9*rng.gen::<f64>(), 
                                0.2, 
                                b as f64 + 0.9*rng.gen::<f64>());
            if (center - point3(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material : Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = rng.gen_range(0., 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
 
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(point3(0., 1., 0.), 1.0, material_1)));

    let material_2 = Rc::new(Lambertian::new(color(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(point3(-4., 1., 0.), 1.0, material_2)));

    let material_3 = Rc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(point3(4., 1., 0.), 1.0, material_3)));
    
    world
}
fn main() {
    let mut rng = thread_rng();
    // Define world
    let world = random_scene();

    // Render
    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio as f32) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut im = ImageBuffer::new(image_width, image_height);

    let lookfrom = point3(13., 2., 3.);
    let lookat = point3(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov_deg = 20.;

    let camera = Camera::new_with_depth_of_field(lookfrom, 
                                                 lookat, 
                                                 vup, 
                                                 vfov_deg, 
                                                 aspect_ratio, 
                                                 aperture, 
                                                 dist_to_focus);

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
    im.save("./13_output.png").unwrap();
}
