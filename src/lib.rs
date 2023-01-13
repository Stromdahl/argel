pub fn fill(pixels: &mut [u32], color: u32) {
    for i in 0..pixels.len() {
        pixels[i] = color;
    }
}

// pub fn set_pixel(pixels: &mut [u32], width: usize, height: usize, x: usize, y: usize, color: u32) {
//     const i = x *
//     pixels[i] = color;
// }

pub fn get_pixel_color(
    pixels: &[u32],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Result<(u8, u8, u8), &'static str> {
    if pixels.is_empty() {
        return Err("Array is empty");
    }

    if x >= width || y >= height {
        return Err("Coordinates are out of bounds");
    }
    let pixel = pixels[y * width + x];
    let red = (pixel >> 16) as u8;
    let green = (pixel >> 8) as u8;
    let blue = pixel as u8;
    Ok((red, green, blue))
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

    #[test]
    fn test_get_pixel_color() {
        let pixels: [u32; 4] = [0xFF0000; 4];
        let result = get_pixel_color(&pixels, 2, 2, 2, 2);
        assert_eq!(Err("Coordinates are out of bounds"), result);

        let pixels: [u32; 4] = [0xFF0000; 4];
        let expected = Ok((255, 0, 0));
        let result = get_pixel_color(&pixels, 0, 0, 2, 2);
        assert_eq!(expected, result);

        let pixels: [u32; 4] = [0x00FF00; 4];
        let expected = Ok((0, 255, 0));
        let result = get_pixel_color(&pixels, 0, 0, 2, 2);
        assert_eq!(expected, result);

        let pixels: [u32; 4] = [0x0000FF; 4];
        let expected = Ok((0, 0, 255));
        let result = get_pixel_color(&pixels, 0, 0, 2, 2);
        assert_eq!(expected, result);

        let pixels: [u32; 4] = [0x0000FF; 4];
        let expected = Ok((0, 0, 255));
        let result = get_pixel_color(&pixels, 0, 0, 2, 2);
        assert_eq!(expected, result);

        let pixels: [u32; 0] = [];
        let result = get_pixel_color(&pixels, 0, 0, 2, 2);
        assert_eq!(Err("Array is empty"), result);
    }
}
