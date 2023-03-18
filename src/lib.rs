#![warn(clippy::all)]
pub mod output;
pub mod color; 

pub mod canvas {
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
}

pub mod rect {
    pub struct NomalizedRect {
        pub x1: i32,
        pub y1: i32,
        pub x2: i32,
        pub y2: i32,
    }

    pub fn nomalize_rect(
        canvas: &crate::canvas::Canvas,
        mut x1: i32,
        mut y1: i32,
        w: i32,
        h: i32,
    ) -> Option<NomalizedRect> {
        if w == 0 || h == 0 {
            return None;
        }

        let mut x2 = x1 + w.signum() * (w.abs() - 1);
        let mut y2 = y1 + h.signum() * (h.abs() - 1);
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        };
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        };

        if x1 < 0 {
            x1 = 0
        };
        if y1 < 0 {
            y1 = 0
        };
        if x2 >= canvas.width as i32 {
            x2 = canvas.width as i32 - 1
        };
        if y2 >= canvas.height as i32 {
            y2 = canvas.height as i32 - 1
        };

        Some(NomalizedRect { x1, y1, x2, y2 })
    }
}

pub fn rect(w: i32, h: i32) -> impl Fn(&mut crate::canvas::Canvas, i32, i32, u32) {
    move |canvas: &mut crate::canvas::Canvas, pos_x, pos_y, color| {
        if let Some(rect) = crate::rect::nomalize_rect(canvas, pos_x, pos_y, w, h) {
            for y in rect.y1..=rect.y2 {
                for x in rect.x1..=rect.x2 {
                    let i: usize = (y * canvas.width as i32 + x) as usize;
                    canvas.pixels[i] = color;
                }
            }
        }
    }
}

pub fn circle(r: i32) -> impl Fn(&mut crate::canvas::Canvas, i32, i32, u32) {
    move |canvas: &mut crate::canvas::Canvas, pos_x, pos_y, color| {
        if let Some(rect) = crate::rect::nomalize_rect(canvas, pos_x, pos_y, r + r, r + r) {
            for y in rect.y1..=rect.y2 {
                for x in rect.x1..=rect.x2 {
                    let dx = x - pos_x - r;
                    let dy = y - pos_y - r;
                    if dx * dx + dy * dy <= r * r {
                        let i: usize = (y * canvas.width as i32 + x) as usize;
                        canvas.pixels[i] = color;
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::rect::nomalize_rect;
    use crate::rect::NomalizedRect;
    fn assert_rect(rect: NomalizedRect, rect_expect: NomalizedRect) {
        assert_eq!(rect.x1, rect_expect.x1);
        assert_eq!(rect.y1, rect_expect.y1);
        assert_eq!(rect.x2, rect_expect.x2);
        assert_eq!(rect.y2, rect_expect.y2);
    }

    #[test]
    fn test_normalize_rect() {
        let canvas = Canvas::new(20, 20);
        let Some(rect) = nomalize_rect(&canvas, 2, 2, 2, 2) else {todo!()};
        let expect: NomalizedRect = NomalizedRect {
            x1: 2,
            y1: 2,
            x2: 3,
            y2: 3,
        };
        assert_rect(rect, expect);

        let canvas = Canvas::new(20, 20);
        let Some(rect) = nomalize_rect(&canvas, 2, 2, -2, -2) else {todo!()};
        let expect: NomalizedRect = NomalizedRect {
            x1: 1,
            y1: 1,
            x2: 2,
            y2: 2,
        };
        assert_rect(rect, expect);

        let canvas = Canvas::new(20, 20);
        let Some(rect) = nomalize_rect(&canvas, 1, 1, -2, -2) else {todo!()};
        let expect: NomalizedRect = NomalizedRect {
            x1: 0,
            y1: 0,
            x2: 1,
            y2: 1,
        };
        assert_rect(rect, expect);

        let canvas = Canvas::new(20, 20);
        let Some(rect) = nomalize_rect(&canvas, 18, 18, 4, 4) else {todo!()};
        let expect: NomalizedRect = NomalizedRect {
            x1: 18,
            y1: 18,
            x2: 19,
            y2: 19,
        };
        assert_rect(rect, expect);
    }

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
            stride: 2,
            pixels: pixels.to_vec(),
        };
        let result = canvas.get_pixel(0, 0);
        assert_eq!(Err("Array is empty"), result);
    }
}
