use std::io::{self, Write, BufWriter};

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
            let r = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let g = (j as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let b: f32 = 0.0;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            stdout.write(format!("{ir} {ig} {ib}\n").as_bytes())?;
        }
    }

    stderr.write("\rDone.                 \n".as_bytes())?;
    stderr.flush()?;
    stdout.flush()?;

    Ok(())
}
