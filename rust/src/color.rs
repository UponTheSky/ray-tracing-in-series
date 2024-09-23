use crate::{util, vec3::Vector3};
use std::io::Write;
use util::interval::Interval;

pub type Color = Vector3;

const INTENSITY: Interval = Interval::new(0.000, 0.999);

pub fn write_color<W>(out: &mut W, pixel_color: &Color) -> std::io::Result<()>
where
    W: Write,
{
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte: i32 = (256.0 * INTENSITY.clamps(r)) as i32;
    let gbyte: i32 = (256.0 * INTENSITY.clamps(g)) as i32;
    let bbyte: i32 = (256.0 * INTENSITY.clamps(b)) as i32;

    out.write(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes())?;

    Ok(())
}
