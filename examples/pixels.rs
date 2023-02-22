extern crate argel;

const IMAGE_PATH:&str = "pixels.ppm";
fn main() {
    const WIDTH:usize = 256;
    const HEIGHT:usize = 256;

    let mut canvas = argel::canvas::Canvas::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = y as u32;
            let g = x as u32;
            let b = 100 as u32;
            let color = r << 16 | g << 8 | b;
            canvas.set_pixel(x, y, color).unwrap();
        }
    }
    argel::save_ppm_image(canvas, IMAGE_PATH).unwrap();
}
