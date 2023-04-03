use crate::vec3::Color;

pub fn convert_color(color: &Color, samples_per_pixel: u32) -> u32 {
    let r = (256.0 * (color.x / (samples_per_pixel as f32)).clamp(0.0, 0.999)) as u32;
    let g = (256.0 * (color.y / (samples_per_pixel as f32)).clamp(0.0, 0.999)) as u32;
    let b = (256.0 * (color.z / (samples_per_pixel as f32)).clamp(0.0, 0.999)) as u32;

    (r << 16) | (g << 8) | b
}
