use image::{EncodableLayout, GenericImageView};
use std::io::{Read, Write};

// Todo! Decoder
pub fn run(input: String, output: String) -> Result<(), String> {
    //Open the image
    let image = match image::open(input) {
        Ok(image) => image,
        Err(e) => {
            return Err(format!("Failed to open image: {}", e));
        }
    };

    // Read the data
    // Green = start of chunk
    // Red = end of chunk
    // White = 1 bit
    // Black = 0 bit
    let mut chunks: Vec<Vec<u8>> = vec![];
    let data_len = (image.width() * image.height()) - 1;
    // Pixel index
    let mut i = 0;
    // Bit mask
    let mut bi = 0;
    let mut temp_chunk = vec![];
    let mut temp_byte: u8 = 0;
    while i <= data_len {
        let (x, y) = get_xy(i as u32, image.width());
        let color = image.get_pixel(x, y);

        // Check for head
        if color.0[0] <= 128 && color.0[1] >= 128 && color.0[2] <= 128 {
            i += 1;
            continue;
        }

        // Check for tail
        if color.0[0] >= 128 && color.0[1] <= 128 && color.0[2] <= 128 {
            i += 1;
            chunks.push(temp_chunk.clone());
            temp_chunk = vec![];
            continue;
        }

        // assemble byte
        if color.0[0] >= 128 && color.0[1] >= 128 && color.0[2] >= 128 {
            temp_byte |= 1 << bi;
        } else {
            temp_byte &= !(1 << bi);
        }

        bi += 1;
        if bi > 7 {
            bi = 0;
            temp_chunk.push(temp_byte);
            temp_byte = 0;
            i += 1;
            continue;
        }
        i += 1
    }

    // Decode the data
    let mut compressed_data: Vec<u8> = vec![];
    for chunk in chunks.iter() {
        match reed_solomon::Decoder::new(8).correct(chunk.as_bytes(), None) {
            Ok(buffer) => {
                compressed_data.extend_from_slice(buffer.data());
            }
            Err(_) => {
                return Err("Failed to decode a chunk, this is likely caused by too many data corruption errors.".to_string());
            }
        };
    }

    // Decompress the data
    let mut decompressed_data = flate2::read::GzDecoder::new(compressed_data.as_slice());
    let mut data = vec![];
    if let Err(e) = decompressed_data.read_to_end(&mut data) {
        return Err(format!("Failed to decompress data: {}", e));
    };

    // Write output file
    let mut file = match std::fs::File::create(output.clone()) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to create output file: {}", e)),
    };

    if let Err(e) = file.write_all(&*data) {
        return Err(format!("Failed to write output file: {}", e));
    }

    file.flush().unwrap();

    println!("Decoded data written to {}", output);
    Ok(())
}

fn get_xy(i: u32, image_size: u32) -> (u32, u32) {
    let x = (i as f32 % image_size as f32) as u32;
    let y = (i as f32 / image_size as f32) as u32;
    (x, y)
}
