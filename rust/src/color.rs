use crate::vec3::Vector3;
use std::io::Write;

pub type Color = Vector3;

pub fn write_color<W>(out: &mut W, pixel_color: &Color) -> std::io::Result<()>
where
    W: Write,
{
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte: i32 = (255.999 * r) as i32;
    let gbyte: i32 = (255.999 * g) as i32;
    let bbyte: i32 = (255.999 * b) as i32;

    out.write(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes())?;

    Ok(())
}
