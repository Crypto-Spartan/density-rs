#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cty;

//include!("bindingsv2.rs");
include!("bindings.rs");

pub fn test_linkage() {
    println!("Link to lib.rs working!");
}

/*
#[cfg(test)]
mod test {
    use super::*;
    use std::mem;

    #[test]
    fn round_trip_compression_decompression() {
        unsafe {
            let text = "This is a simple example on how to use the simple Density API.".as_bytes();
            let text_length = text.len() as uint_fast64_t;
            let text_ptr = text.as_ptr();

            let compress_safe_size = density_compress_safe_size(text_length);
            let decompress_safe_size = density_decompress_safe_size(text_length);

            let mut compressed_output: Vec<u8> = vec![0; text.len()];
            let mut decompressed_output: Vec<u8> = vec![0; text.len()];
            let mut compressed_output_ptr = compressed_output.as_mut_ptr();
            let mut decompressed_output_ptr = decompressed_output.as_mut_ptr();

            let result: density_processing_result = density_compress(text_ptr, text_length, compressed_output_ptr, compress_safe_size, DENSITY_ALGORITHM_DENSITY_ALGORITHM_CHAMELEON);

            /*if result.state == DENSITY_STATE_DENSITY_STATE_OK {
                println!("Compressed {} bytes to {} bytes", result.bytesRead, result.bytesWritten);
            } else {
                panic!("Compression Error");
            }*/

            //let result: density_processing_result = density_decompress(compressed_output_ptr, result.bytesWritten, decompressed_output_ptr, decompress_safe_size);

            /*if result.state == DENSITY_STATE_DENSITY_STATE_OK {
                println!("Deompressed {} bytes to {} bytes", result.bytesRead, result.bytesWritten);
            } else {
                panic!("Deompression Error");
            }*/

            //assert!(text == decompressed_output);

            /*pub fn density_compress(
                input_buffer: *const u8,
                input_size: uint_fast64_t,
                output_buffer: *mut u8,
                output_size: uint_fast64_t,
                algorithm: DENSITY_ALGORITHM,
            ) -> density_processing_result*/
            
        }
    }
        
}*/