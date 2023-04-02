use crate::vec3::Color;

pub fn convert_color(color: Color) -> u32 {
    (((255.999 * color.x) as u32) << 16)
        | (((255.999 * color.y) as u32) << 8)
        | (255.999 * color.z) as u32
}
