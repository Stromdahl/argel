use std::io::Write;
pub fn save_ppm_image(pixels: Vec<u32>, width: usize, height: usize, path: &str) -> std::io::Result<()> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    let header = format!("P6\n{} {}\n255\n", width, height);
    file.write_all(header.as_bytes())?;
    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = pixels[j * width + i];
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            let color = [r, g, b];
            file.write_all(&color)?;
        }
    }
    file.flush()?;
    Ok(())
}
