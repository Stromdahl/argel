extern crate argel;
const IMAGE_PATH: &str = "pixels.ppm";

const WIDTH: usize = 255;
const HEIGHT: usize = 255;

fn main() {
    let mut canvas = argel::canvas::Canvas::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = y as u32;
            let g = x as u32;
            let b = 100_u32;
            let color = r << 16 | g << 8 | b;
            canvas.set_pixel(x, y, color).unwrap();
        }
    }
    argel::output::save_ppm_image(canvas.pixels, canvas.width, canvas.height, IMAGE_PATH).unwrap();
}
