extern crate argel;

fn main() {
    let mut canvas = argel::Canvas::new(100, 100);
    canvas.fill(0xFF0000);
    canvas.draw(25,25, argel::rect(50, 50), 0x00FF00);
    let path = "foo.ppm";
    argel::save_ppm_image(canvas, path).unwrap();
}
