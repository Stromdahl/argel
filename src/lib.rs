pub fn fill(pixels: &mut [u32], color: u32) {
    for i in 0..pixels.len() {
        pixels[i] = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_fill_all_pixels() {
        let mut pixels: [u32; 4] = [0; 4];
        let color = 0x0000FF;
        fill(&mut pixels, color);
        for x in pixels {
            assert_eq!(x, color);
        }
    }
}
