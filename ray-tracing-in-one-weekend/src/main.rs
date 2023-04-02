use std::vec;

use color::convert_color;
use ray::Ray;
use softbuffer::GraphicsContext;
use vec3::{dot, unit_vector, Color};
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

use crate::vec3::{Point3, Vec3};

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - center;
    let a = dot(r.direction, r.direction);
    let b = 2.0 * dot(oc, r.direction);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }

    let unit_direction = unit_vector(r.direction);
    t = (unit_direction.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,
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
                            let u = i as f32 / (image_width - 1) as f32;
                            let v = j as f32 / (image_height - 1) as f32;
                            let r = Ray::new(
                                origin,
                                lower_left_corner + horizontal * u + vertical * v - origin,
                            );
                            let pixel_color = ray_color(&r);
                            buffer[((image_height - j - 1) * image_width + i) as usize] =
                                convert_color(pixel_color);
                        }
                    }
                    graphics_context.set_buffer(&buffer, image_width as u16, image_height as u16);
                    redraw = false;
                }
            }
            _ => (),
        }
    });
}
