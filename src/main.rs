use density_rs::*; 
use std::mem;



fn main() {
    unsafe {
        round_trip_compression_decompression();
    }
}


unsafe fn round_trip_compression_decompression() {
    let text = "This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+".to_owned();

    let text_bytes = text.as_bytes();
    println!("text_bytes = {}", std::str::from_utf8(text_bytes.clone()).unwrap());
    let text_length = text_bytes.len();
    println!("text_length = {}", text_length);

    let compress_safe_size = density_compress_safe_size(text_length as _);
    let decompress_safe_size = density_decompress_safe_size(text_length as _);
    println!(
        "compress_safe_size = {}\ndecompress_safe_size = {}",
        compress_safe_size, decompress_safe_size
    );

    let mut compressed_output: Vec<u8> = vec![0; compress_safe_size as usize];
    let mut decompressed_output: Vec<u8> = vec![0; decompress_safe_size as usize];

    let result: density_processing_result = density_compress(
        text_bytes.as_ptr() as _, 
        text_length as _, 
        compressed_output.as_mut_ptr() as *mut _, 
        compress_safe_size as _, 
        DENSITY_ALGORITHM_DENSITY_ALGORITHM_CHAMELEON
    );

    if result.state == DENSITY_STATE_DENSITY_STATE_OK {
        println!("Compressed {} bytes to {} bytes", result.bytesRead, result.bytesWritten);
    } else {
        //result.state is a u32
        let error = read_density_error(result.state);
        panic!("Compression Error: {}", error);
    }

    let result: density_processing_result = density_decompress(
        compressed_output.as_mut_ptr() as *mut _, 
        result.bytesWritten as _, 
        decompressed_output.as_mut_ptr() as *mut _,
        decompress_safe_size as _
    );

    if result.state == DENSITY_STATE_DENSITY_STATE_OK {
        println!("Deompressed {} bytes to {} bytes", result.bytesRead, result.bytesWritten);
        decompressed_output.truncate(result.bytesWritten as _);
    } else {
        let error = read_density_error(result.state);
        panic!("Deompression Error: {}", error);
    }

    println!("text_bytes = {:?}\ndecompressed_output = {:?}", text_bytes, decompressed_output);
    assert!(text_bytes == &decompressed_output);

    // c function signatures for reference

    /*pub fn density_compress(
        input_buffer: *const u8,
        input_size: uint_fast64_t,
        output_buffer: *mut u8,
        output_size: uint_fast64_t,
        algorithm: DENSITY_ALGORITHM,
    ) -> density_processing_result*/

    /*pub fn density_decompress(
        input_buffer: *const u8,
        input_size: uint_fast64_t,
        output_buffer: *mut u8,
        output_size: uint_fast64_t,
    ) -> density_processing_result;*/
        
}

fn read_density_error(err_code: u32) -> String {
    match err_code {
        1u32 => "DENSITY_STATE_ERROR_INPUT_BUFFER_TOO_SMALL".to_owned(),
        2u32 => "DENSITY_STATE_ERROR_OUTPUT_BUFFER_TOO_SMALL".to_owned(),
        3u32 => "DENSITY_STATE_ERROR_DURING_PROCESSING".to_owned(),
        4u32 => "DENSITY_STATE_ERROR_INVALID_CONTEXT".to_owned(),
        5u32 => "DENSITY_STATE_ERROR_INVALID_ALGORITHM".to_owned(),
        _ => unreachable!()
    }
}