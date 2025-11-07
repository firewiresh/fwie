use flate2::write::GzEncoder;
use imageproc::image::{ColorType, DynamicImage, GenericImage, Rgba};
use indicatif::{ProgressBar, ProgressStyle};
use reed_solomon::Encoder;
use std::io::{Read, Write};

pub fn run(input: String, output: String) -> Result<(), String> {
    // Read the input file
    let mut file = match std::fs::File::open(input) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open input file: {}", e)),
    };
    let mut data = Vec::new();
    if let Err(e) = file.read_to_end(&mut data) {
        return Err(format!("Failed to read input file: {}", e));
    }
    // GZ compress the data
    let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::default());
    if let Err(e) = encoder.write_all(&data) {
        return Err(format!("Failed to compress input file: {}", e));
    };

    let compressed_data = match encoder.finish() {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to compress input file: {}", e)),
    };

    // Run reed solomon encoding on it
    let mut chunks: Vec<Vec<u8>> = vec![];
    let pb = ProgressBar::new(compressed_data.len() as u64 / 248);
    pb.set_message("Letting the cat eat a few bytes");
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
        .expect("Failed to create template")
        .progress_chars("#>-"));

    // Todo! Parallelism
    for chunk in compressed_data.chunks(247) {
        let buffer = Encoder::new(8).encode(chunk);
        let encoded_chunk = buffer[..].to_vec();
        chunks.push(encoded_chunk);
        pb.inc(1);
    }
    pb.finish_and_clear();

    // Encode to an image
    let mut data_len = 0;
    for chunk in chunks.iter() {
        data_len += (chunk.len() * 8) + 2;
    }
    let image_size = (data_len as f64).sqrt().ceil() as u32;

    let mut image = DynamicImage::new(image_size, image_size, ColorType::Rgb8);

    let mut i = 0;
    let pb = ProgressBar::new(chunks.len() as u64);
    pb.set_message("What are pixels anyways?");
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
        .expect("Failed to create template")
        .progress_chars("#>-"));
    for chunk in chunks {
        // Write the chunk head
        let (chunk_head_x, chunk_head_y) = get_xy(i as u32, image_size);
        image.put_pixel(chunk_head_x, chunk_head_y, Rgba([0, 255, 0, 255]));
        i += 1;
        for byte in chunk.iter() {
            for m in 0..8 {
                let is_set = byte >> m & 1;
                let (x, y) = get_xy(i as u32, image_size);
                image.put_pixel(
                    x,
                    y,
                    if is_set == 1 {
                        Rgba([255, 255, 255, 255])
                    } else {
                        Rgba([0, 0, 0, 255])
                    },
                );
                i += 1;
            }
        }
        // Write the chunk tail
        let (chunk_tail_x, chunk_tail_y) = get_xy(i as u32, image_size);
        image.put_pixel(chunk_tail_x, chunk_tail_y, Rgba([255, 0, 0, 255]));
        i += 1;
        pb.inc(1);
    }
    pb.finish_and_clear();

    // Save the image
    if let Err(e) = image.save(output.clone()) {
        return Err(format!("Failed to save image: {}", e));
    };

    println!("Image saved to {}", output);
    Ok(())
}

// Function to get the XY coord from a pixel index
fn get_xy(i: u32, image_size: u32) -> (u32, u32) {
    let x = (i as f32 % image_size as f32) as u32;
    let y = (i as f32 / image_size as f32) as u32;
    (x, y)
}
