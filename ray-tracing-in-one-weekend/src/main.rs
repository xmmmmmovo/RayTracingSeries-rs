use std::{sync::Mutex, vec};

use hittable::{Hittable, HittableList};
use rand::Rng;
use ray::Ray;
use softbuffer::GraphicsContext;
use sphere::Sphere;
use vec3::{random, Color};
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
    vec3::{convert_color, Point3, Vec3},
};

use rayon::prelude::*;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

fn ray_color(ray: &Ray, world: &hittable::HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            ray_color(&scattered, world, depth - 1) * attenuation
        } else {
            Color::ZERO
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();

    let ground_mat = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = random(0.0..1.0) * random(0.0..1.0);
                let sphere_mat = Box::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Box::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Box::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Box::new(Dielectric::new(1.5));
    let mat2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    let image_width = 800;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;
    let mut samples_per_pixel: u32 = 1;
    const MAX_DEPTH: u32 = 50;

    // World
    let world: hittable::HittableList = random_scene();

    // Camera
    let mut lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let mut camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    let buffer = Mutex::new(vec![0u32; (image_width * image_height) as usize]);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Image Window")
        .with_inner_size(winit::dpi::LogicalSize::new(image_width, image_height))
        .build(&event_loop)
        .unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    let mut redraw = true;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(virtual_keycode) = input.virtual_keycode {
                        if input.state == ElementState::Released {
                            return;
                        }

                        if virtual_keycode == VirtualKeyCode::Escape {
                            *control_flow = winit::event_loop::ControlFlow::Exit;
                            return;
                        }

                        if redraw {
                            return;
                        }

                        if virtual_keycode == VirtualKeyCode::W {
                            lookfrom.z -= 1.0;
                            redraw = true;
                        } else if virtual_keycode == VirtualKeyCode::S {
                            lookfrom.z += 1.0;
                            redraw = true;
                        } else if virtual_keycode == VirtualKeyCode::A {
                            lookfrom.x -= 1.0;
                            redraw = true;
                        } else if virtual_keycode == VirtualKeyCode::D {
                            lookfrom.x += 1.0;
                            redraw = true;
                        }

                        if redraw {
                            println!("lookfrom: {:?}", lookfrom);
                            camera = Camera::new(
                                lookfrom,
                                lookat,
                                vup,
                                20.0,
                                ASPECT_RATIO,
                                aperture,
                                dist_to_focus,
                            )
                        }

                        if virtual_keycode == VirtualKeyCode::U {
                            samples_per_pixel *= 10;
                            if samples_per_pixel >= 500 {
                                samples_per_pixel = 500;
                            }
                            redraw = true;
                        } else if virtual_keycode == VirtualKeyCode::I {
                            samples_per_pixel /= 10;
                            if samples_per_pixel == 0 {
                                samples_per_pixel = 1;
                            }
                            redraw = true;
                        }
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                if redraw {
                    println!("start draw..., spp: {}", samples_per_pixel);
                    for j in (0..image_height).rev() {
                        (0..image_width).into_par_iter().for_each(|i| {
                            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                            let mut rng = rand::thread_rng();
                            for _ in 0..samples_per_pixel {
                                let u =
                                    ((i as f64) + rng.gen::<f64>()) / ((image_width - 1) as f64);
                                let v =
                                    ((j as f64) + rng.gen::<f64>()) / ((image_height - 1) as f64);
                                let r = camera.get_ray(u, v);
                                pixel_color += ray_color(&r, &world, MAX_DEPTH);
                            }
                            buffer.lock().unwrap()
                                [((image_height - j - 1) * image_width + i) as usize] =
                                convert_color(&pixel_color, samples_per_pixel);
                        });
                    }
                    println!("Done.");
                    graphics_context.set_buffer(
                        &(buffer.lock().unwrap()),
                        image_width as u16,
                        image_height as u16,
                    );
                    redraw = false;
                }
            }
            _ => (),
        }
    });
}
