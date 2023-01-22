## Argel
Argel is a graphics library for rust with the goal to be as dependency light as posible.
Work in progress and a learning-rust-project

## Usage
`cargo run --example simple`
``` Rust
extern crate argel;

fn main() {
    let mut canvas = argel::Canvas::new(100, 100);
    canvas.fill(0xFF0000);
    canvas.draw(25,25, argel::rect(50, 50), 0x00FF00);
    let path = "foo.ppm";
    argel::save_ppm_image(canvas, path).unwrap();
}
```
### Examples
`cargo run --example <example>`
* simple - renders canvas as a ppm image
