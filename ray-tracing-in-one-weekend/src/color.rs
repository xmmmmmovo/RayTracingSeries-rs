use std::io::Write;

use crate::vec3::Color;

pub fn write_color(write: &mut dyn Write, color: Color) -> Result<(), std::io::Error> {
    writeln!(
        write,
        "{} {} {}",
        (255.999 * color.x) as i32,
        (255.999 * color.y) as i32,
        (255.999 * color.z) as i32
    )?;
    Ok(())
}
