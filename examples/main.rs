extern crate argel;

const WIDTH:usize = 400;
const HEIGHT:usize = 400;

struct Canvas<'a> {
    width: usize,
    height: usize,
    pixels: &'a mut [u32],
}

impl Canvas<'_> {
    fn fill(mut self, color: u32) {
        argel::fill(&mut self.pixels, color);
    }
}

fn main() {

    let mut pixels: [u32; HEIGHT * WIDTH] = [0; HEIGHT * WIDTH];

    let canvas: Canvas = Canvas {
        width: WIDTH,
        height: HEIGHT,
        pixels: &mut pixels,
    };

    canvas.fill(0x0000FF);
    println!("{}", canvas.width);
    println!("{}", canvas.height);
}
