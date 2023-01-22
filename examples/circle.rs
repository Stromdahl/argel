extern crate argel;

fn main() {
    let mut canvas = argel::Canvas::new(100, 100);
    canvas.draw(50,50, argel::circle(25), 0x00FF00);
    let path = "circle.ppm";
    argel::save_ppm_image(canvas, path).unwrap();
}
