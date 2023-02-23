use std::io::Write;
const BUFFER_SIZE:usize = 1024;

pub fn save_ppm_image(
    pixels: Vec<u32>,
    width: usize,
    height: usize,
    path: &str,
) -> std::io::Result<()> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    let header = format!("P6 {} {} 255\n", width, height);
    file.write_all(header.as_bytes())?;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut index = 0;
    for pixel in &pixels {
        if index + 3 >= buffer.len() {
            file.write_all(&buffer[0..index])?;
            index = 0;
        }

        let r = ((pixel >> 16) & 0xFF) as u8;
        let g = ((pixel >> 8) & 0xFF) as u8;
        let b = (pixel & 0xFF) as u8;

        buffer[index] = r;
        buffer[index + 1] = g;
        buffer[index + 2] = b;

        index += 3;
    }
    if index > 0 {
        file.write_all(&buffer[0..index])?;
    }
    Ok(())
}
