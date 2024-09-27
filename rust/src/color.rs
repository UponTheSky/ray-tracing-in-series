use crate::{util, vec3::Vector3};
use std::io::Write;
use util::interval::Interval;

pub type Color = Vector3;

#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        f64::sqrt(linear_component)
    } else {
        0.0
    }
}

const INTENSITY: Interval = Interval::new(0.000, 0.999);

pub fn write_color<W>(out: &mut W, pixel_color: &Color) -> std::io::Result<()>
where
    W: Write,
{
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let rbyte: i32 = (256.0 * INTENSITY.clamps(r)) as i32;
    let gbyte: i32 = (256.0 * INTENSITY.clamps(g)) as i32;
    let bbyte: i32 = (256.0 * INTENSITY.clamps(b)) as i32;

    out.write(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes())?;

    Ok(())
}
