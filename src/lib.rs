#![warn(clippy::all)]
use std::{fs::File, io::Write};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub pixels: Vec<u32>,
}

impl Canvas {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![0; width * height];
        Canvas {
            width,
            height,
            stride: width,
            pixels,
        }
    }

    pub fn fill(&mut self, color: u32) {
        for i in 0..self.width * self.height {
            self.pixels[i] = color;
        }
    }

    pub fn draw<F: Fn(&mut Canvas, i32, i32, u32)>(
        &mut self,
        x: i32,
        y: i32,
        draw_fn: F,
        color: u32,
    ) {
        draw_fn(self, x, y, color);
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<(u8, u8, u8), &'static str> {
        assert_pixel(&self.pixels, self.width, self.height, x, y)?;

        let pixel = self.pixels[y * self.width + x];
        let red = (pixel >> 16) as u8;
        let green = (pixel >> 8) as u8;
        let blue = pixel as u8;
        Ok((red, green, blue))
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), &str> {
        assert_pixel(&self.pixels, self.width, self.height, x, y)?;

        self.pixels[y * self.width + x] = color;
        Ok(())
    }
}

pub fn rect(w: i32, h: i32) -> impl Fn(&mut Canvas, i32, i32, u32) {
    move |canvas: &mut Canvas, mut x1, mut y1, color| {
        let mut x2 = x1 + w.signum() * (w.abs() - 1);
        let mut y2 = y1 + h.signum() * (h.abs() - 1);
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        };
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        };

        // TODO: Make this better, it's not pretty
        for y in y1..=y2 {
            if 0 <= y && y < canvas.height as i32 {
                for x in x1..=x2 {
                    if 0 <= x && x < canvas.width as i32 {
                        let i: usize = (y * canvas.width as i32 + x) as usize;
                        canvas.pixels[i] = color;
                    }
                }
            }
        }
    }
}

pub fn circle(r: i32) -> impl Fn(&mut Canvas, i32, i32, u32) {
    move |canvas: &mut Canvas, pos_x, pos_y, color| {
        let x1 = pos_x - r;
        let y1 = pos_y - r;
        let x2 = pos_x + r;
        let y2 = pos_y + r;

        // TODO: Make this better, it's not pretty
        for y in y1..=y2 {
            if 0 <= y && y < canvas.height as i32 {
                for x in x1..=x2 {
                    if 0 <= x && x < canvas.width as i32 {
                        let dx = x - pos_x;
                        let dy = y - pos_y;
                        if dx * dx + dy * dy <= r * r {
                            let i: usize = (y * canvas.width as i32 + x) as usize;
                            canvas.pixels[i] = color;
                        }
                    }
                }
            }
        }
    }
}

fn assert_pixel(
    pixels: &[u32],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
) -> Result<(), &'static str> {
    if pixels.is_empty() {
        return Err("Array is empty");
    }

    if x >= width || y >= height {
        return Err("Coordinates are out of bounds");
    }

    Ok(())
}

pub fn save_ppm_image(canvas: Canvas, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let header = format!("P6 {} {} 255\n", canvas.width, canvas.height);
    file.write_all(header.as_bytes())?;
    for pixel in &canvas.pixels {
        let r = ((pixel >> 16) & 0xFF) as u8;
        let g = ((pixel >> 8) & 0xFF) as u8;
        let b = (pixel & 0xFF) as u8;
        file.write_all(&[r, g, b])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_fill() {
        let mut canvas = Canvas::new(2, 2);
        let color = 0xFF0000;
        canvas.fill(color);
        for i in 0..canvas.pixels.len() {
            assert_eq!(canvas.pixels[i], color);
        }
    }

    //    #[test]
    //    fn test_draw_rect_() {
    //        let mut pixels: [u32; 4] = [0; 4];
    //        let color = 0xff0000;
    //
    //        draw(&mut pixels,2 ,2, rect(0, 0, 2, 2));
    //        for x in pixels {
    //            assert_eq!(x, color);
    //        }
    //    }

    #[test]
    fn test_set_pixel() {
        let mut canvas = Canvas::new(2, 2);
        let result = canvas.set_pixel(2, 2, 0xFF0000);
        assert_eq!(Err("Coordinates are out of bounds"), result);

        let expected = Ok((255, 0, 0));
        let mut canvas = Canvas::new(2, 2);
        let set_result = canvas.set_pixel(0, 0, 0xFF0000);
        assert_eq!(Ok(()), set_result);

        let result = canvas.get_pixel(0, 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_pixel() {
        let canvas = Canvas::new(2, 2);
        let result = canvas.get_pixel(2, 2);
        assert_eq!(Err("Coordinates are out of bounds"), result);

        let mut canvas = Canvas::new(2, 2);
        canvas.fill(0xFF0000);
        let expected = Ok((255, 0, 0));
        let result = canvas.get_pixel(0, 0);
        assert_eq!(expected, result);

        let mut canvas = Canvas::new(2, 2);
        canvas.fill(0x00FF00);
        let expected = Ok((0, 255, 0));
        let result = canvas.get_pixel(0, 0);
        assert_eq!(expected, result);

        let mut canvas = Canvas::new(2, 2);
        canvas.fill(0x0000FF);
        let expected = Ok((0, 0, 255));
        let result = canvas.get_pixel(0, 0);
        assert_eq!(expected, result);

        let pixels: [u32; 0] = [];
        let canvas = Canvas {
            width: 2,
            height: 2,
            pixels: pixels.to_vec(),
        };
        let result = canvas.get_pixel(0, 0);
        assert_eq!(Err("Array is empty"), result);
    }
}
