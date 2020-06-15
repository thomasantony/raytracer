use std::sync::Arc;
use rand::prelude::*;
use raytracer::{color, point3, Vec3, Camera, Color, HittableList, Sphere};
use raytracer::materials::{Material, Lambertian, Metal, Dielectric};
use raytracer::renderers::{Renderer, SimpleRenderer, RayonRenderer};

fn random_scene() -> HittableList {
    let mut rng = thread_rng();
    let mut world = HittableList::new();
    
    let ground_material = Arc::new(Lambertian::new(color(0.5, 0.5, 0.5)));
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
                let sphere_material : Arc<dyn Material + Sync + Send> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = rng.gen_range(0., 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
 
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(point3(0., 1., 0.), 1.0, material_1)));

    let material_2 = Arc::new(Lambertian::new(color(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(point3(-4., 1., 0.), 1.0, material_2)));

    let material_3 = Arc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(point3(4., 1., 0.), 1.0, material_3)));
    
    world
}

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "13_final_scene", about = "Final scene.")]
struct Opt {
    /// Use parallel renderer
    #[structopt(short = "p", long)]
    parallel: bool,

    /// Set image width
    #[structopt(short = "w", long = "velocity", default_value = "384")]
    image_width: u32,
}

fn main() {
    let opt = Opt::from_args();

    // Define world
    let world = random_scene();

    // Render
    let aspect_ratio = 16. / 9.;
    let image_width = opt.image_width;
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

    let im = if !opt.parallel
    {
        let renderer = SimpleRenderer::default();
        renderer.render(world, &camera, image_width, image_height, samples_per_pixel, max_depth)
    }else{
        let renderer = RayonRenderer::default();
        renderer.render(world, &camera, image_width, image_height, samples_per_pixel, max_depth)
    };
    
    println!("");
    im.save("./13_output.png").unwrap();
}
