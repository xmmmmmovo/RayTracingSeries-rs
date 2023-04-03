use std::vec;

use color::convert_color;
use hittable::Hittable;
use rand::Rng;
use ray::Ray;
use softbuffer::GraphicsContext;
use sphere::Sphere;
use vec3::{unit_vector, Color};
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{
    camera::Camera,
    vec3::{Point3, Vec3},
};

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

fn ray_color(ray: &Ray, world: &hittable::HittableList) -> Color {
    // 球在(0,0,-1)处，半径为0.5
    if let Some(rec) = world.hit(ray, 0.0, f32::INFINITY) {
        (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit_direction = unit_vector(ray.direction);
        let t = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World
    let world: hittable::HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    // Render
    let mut buffer = vec![0u32; (image_width * image_height) as usize];

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
                        if virtual_keycode == VirtualKeyCode::Escape {
                            *control_flow = winit::event_loop::ControlFlow::Exit;
                        } else if virtual_keycode == VirtualKeyCode::W {
                            // TODO: 需要更新view矩阵
                            redraw = true;
                        } else if virtual_keycode == VirtualKeyCode::S {
                            redraw = true;
                        } else if virtual_keycode == VirtualKeyCode::A {
                            redraw = true;
                        } else if virtual_keycode == VirtualKeyCode::D {
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
                    for j in (0..image_height).rev() {
                        for i in 0..image_width {
                            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                            for _ in 0..SAMPLES_PER_PIXEL {
                                let u =
                                    ((i as f32) + rng.gen::<f32>()) / ((image_width - 1) as f32);
                                let v =
                                    ((j as f32) + rng.gen::<f32>()) / ((image_height - 1) as f32);

                                let r = camera.get_ray(u, v);
                                pixel_color += ray_color(&r, &world);
                            }
                            buffer[((image_height - j - 1) * image_width + i) as usize] =
                                convert_color(&pixel_color, SAMPLES_PER_PIXEL);
                        }
                    }
                    graphics_context.set_buffer(&buffer, image_width as u16, image_height as u16);
                    redraw = false;
                }
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable to render in this event rather than in MainEventsCleared, since
                // rendering in here allows the program to gracefully handle redraws requested
                // by the OS.
                //
                // If you're trying to animate something and need to redraw at a consistent rate,
                // consider using the `request_redraw` method on `ControlFlow`.
                println!("Redraw");
            }
            _ => (),
        }
    });
}
