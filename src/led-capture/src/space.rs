use crate::colors::Rgb;

pub struct Matrix<'a> {
    pub width: u32,
    pub height: u32,
    buffer: &'a [u8]
}

pub struct Position {
    pub x: u32,
    pub y: u32
}

impl<'a, 'c> Matrix<'a> {
    pub fn new(width: u32, height: u32, buffer: &[u8]) -> Matrix {
        Matrix {
            width,
            height,
            buffer: &buffer
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Rgb> {
        if x > self.width || y > self.height {
            return None;
        }
        let start: usize = ((self.width * 3) * y + (3 * x)) as usize;
        if start + 3 >= self.buffer.len() {
            return None;
        }
        Some(Rgb {
            r: self.buffer[start],
            g: self.buffer[start + 1],
            b: self.buffer[start + 2]
        })
    }

    pub fn get_avg_color(&self, x: u32, y: u32, width: u32, height: u32) -> Rgb {
        let mut sum_r: u32 = 0;
        let mut sum_g: u32 = 0;
        let mut sum_b: u32 = 0;
        let mut count: u32 = 0;

        for i in y..(y + height) {
            for j in x..(x + width) {
                if let Some(pixel) = self.get_pixel(j, i) {
                    sum_r += pixel.r as u32;
                    sum_g += pixel.g as u32;
                    sum_b += pixel.b as u32;
                    count += 1;
                }
            }
        }

        if count == 0 {
            return Rgb { r: 0, g: 0, b: 0 };
        }

        Rgb {
            r: (sum_r / count) as u8,
            g: (sum_g / count) as u8,
            b: (sum_b / count) as u8,
        }
    }
}
