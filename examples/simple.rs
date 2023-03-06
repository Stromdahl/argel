extern crate argel;

const IMAGE_PATH:&str = "simple.ppm";
fn main() {
    let mut canvas = argel::canvas::Canvas::new(100, 100);
    let color = 0x00FF00;
    canvas.fill(0x000000);
    canvas.draw(0, 0, argel::rect(25, 25), color);
    canvas.draw(0, 75, argel::rect(25, 25), color);
    canvas.draw(75, 75, argel::rect(25, 25), color);
    canvas.draw(75, 0, argel::rect(25, 25), color);
    canvas.draw(25, 25, argel::rect(50, 50), color);
    argel::output::save_ppm_image(canvas.pixels,canvas.width, canvas.height, IMAGE_PATH).unwrap();
}
