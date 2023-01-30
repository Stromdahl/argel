use std::time::Instant;

extern crate argel;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    const R: i32 = 50;
    let mut canvas = argel::Canvas::new(WIDTH, HEIGHT);
    let mut count = 0;
    let now = Instant::now();
    for y in 0..HEIGHT as i32 / R / 2 {
        for x in 0..WIDTH as i32 / R / 2 {
            if count % 2 == 0 {
                canvas.draw(x * R * 2, y * R * 2, argel::circle(R), 0x00FF00);
            }
            count += 1;
        }
        count += 1;
    }
    let elapsed_time = now.elapsed();
    println!(
        "Circles done after {}ms",
        elapsed_time.as_millis()
    );
    let path = "circle.ppm";
    argel::save_ppm_image(canvas, path).unwrap();
    let elapsed_time = now.elapsed();
    println!(
        "Saving done after {}ms",
        elapsed_time.as_millis()
    );
}
