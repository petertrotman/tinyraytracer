use std::io::prelude::*;
use std::vec::Vec;

#[derive(Copy, Clone, Debug, Default)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl std::iter::IntoIterator for &Pixel {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.r, self.g, self.b].into_iter()
    }
}

#[derive(Debug)]
struct Frame {
    width: usize,
    height: usize,
    buffer: Vec<Pixel>,
}

impl Frame {
    fn new(width: usize, height: usize) -> Frame {
        Frame {
            width,
            height,
            buffer: vec![Pixel::default(); width * height],
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut frame = Frame::new(1024, 768);

    frame.buffer = (0..frame.height)
        .flat_map(|i| {
            (0..frame.width).map(move |j| Pixel {
                r: (i * 255 / frame.height) as u8,
                g: (j * 255 / frame.width) as u8,
                b: 0,
            })
        })
        .collect();

    let mut file = std::fs::File::create("out.ppm")?;
    writeln!(file, "P6 {} {} 255", frame.width, frame.height)?;
    file.write_all(&frame.buffer.iter().flatten().collect::<Vec<u8>>())?;

    Ok(())
}
