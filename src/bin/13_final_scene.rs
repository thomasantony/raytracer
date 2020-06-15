use std::rc::Rc;
use rand::prelude::*;
use raytracer::{color, point3, Vec3, Camera, Color, HittableList, Sphere};
use raytracer::materials::{Material, Lambertian, Metal, Dielectric};
use raytracer::renderers::{Renderer, SimpleRenderer};

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
    // Define world
    let world = random_scene();

    // Render
    let aspect_ratio = 16. / 9.;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio as f32) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

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

    let renderer = SimpleRenderer::default();
    let im = renderer.render(&world, &camera, image_width, image_height, samples_per_pixel, max_depth);
    println!("");
    im.save("./13_output.png").unwrap();
}
