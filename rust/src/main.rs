mod color;
mod point;
mod ray;
mod vec3;

use color::{write_color, Color};
use std::io::{self, BufWriter, Write};

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() -> std::io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut stderr = BufWriter::new(io::stderr().lock());

    stdout.write(b"P3\n")?;
    stdout.write((format!("{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")).as_bytes())?;

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            stderr.write(format!("\rScanlines remaining: {} ", IMAGE_HEIGHT - j).as_bytes())?;
            stderr.flush()?;

            let color = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.0,
            );
            write_color(&mut stdout, &color)?;
        }
    }

    stderr.write("\rDone.                 \n".as_bytes())?;
    stderr.flush()?;
    stdout.flush()?;

    Ok(())
}
