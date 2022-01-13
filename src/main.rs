#![allow(unused_imports)]

use density_rs::{
    c_bindings,
    compress_block, decompress_block,
    DensityResult, DensityState, DensityAlgorithm
};
use rand::prelude::*;

/*use density_rs::{max_2, max_3};
//use std::cmp;

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

fn main() {
    let x = 100;
    let y = 101;
    let z = 102;
    let res = max_3!(x, y, z);
    println!("Max of (x, y): {:?}", res);
    println!("Max of (x, y): {:?}", max_2!(x,y));
}*/

fn main() {
    test_round_trip_compression_decompression();
    
    /*use std::alloc::{alloc, Layout};
    let mut dictionary = {
        let layout = Layout::new::<[u32; 65_535]>();
        let mut buf = unsafe {
            let ptr = alloc(layout) as *mut [u32; 65_535];
            Box::from_raw(ptr)
        };
        buf
    };
    if dictionary.into_iter().all(|x| x == 0) {
        println!("True!");
    } else {
        println!("False!");
    }*/

    /*unsafe {
        let chameleon_size: usize = c_bindings::density_get_dictionary_size(c_bindings::DENSITY_ALGORITHM_DENSITY_ALGORITHM_CHAMELEON);
        let cheetah_size: usize = c_bindings::density_get_dictionary_size(c_bindings::DENSITY_ALGORITHM_DENSITY_ALGORITHM_CHEETAH);
        let lion_size: usize = c_bindings::density_get_dictionary_size(c_bindings::DENSITY_ALGORITHM_DENSITY_ALGORITHM_LION);
        println!("chameleon_size: {}", chameleon_size);
        println!("cheetah_size: {}", cheetah_size);
        println!("lion_size: {}", lion_size);
    }*/
}

fn test_round_trip_compression_decompression() {
    
    let text = "This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+".to_owned();

    /*const data_len: usize = 262_144;
    let mut text = vec![0u8; data_len];
    let mut seeded_rng = StdRng::seed_from_u64(510152025);

    for i in text.iter_mut() {
        *i = seeded_rng.gen();
    }*/

    //dbg!(text);


    let text_bytes = text.as_bytes();
    //let text_bytes = &text[..];
    //println!("text_bytes = {}", std::str::from_utf8(text_bytes.clone()).unwrap());
    let text_length = text_bytes.len();
    println!("text_length = {}", text_length);

    //let compress_safe_size;
    let decompress_safe_size;

    let num_chunks_256 = text_length >> 8;
    let compress_safe_size = ((num_chunks_256+1) * 320) + 8;

    unsafe {
        //compress_safe_size = c_bindings::density_compress_safe_size(text_length as _);
        decompress_safe_size = c_bindings::density_decompress_safe_size(text_length as _);
    }
    println!(
        "compress_safe_size = {}\ndecompress_safe_size = {}",
        compress_safe_size, decompress_safe_size
    );

    let mut compressed_output = vec![0u8; compress_safe_size as usize];
    let mut decompressed_output = vec![0u8; decompress_safe_size as usize];

    //let result: c_bindings::density_processing_result = compress_block(
    let result: DensityResult = compress_block(
        &text_bytes, 
        &mut compressed_output,
        DensityAlgorithm::Chameleon
    );

    if matches!(result.state, DensityState::OK) {
        println!("Compressed {} bytes to {} bytes", result.bytes_read, result.bytes_written);
        compressed_output.truncate(result.bytes_written);
    } else {
        //result.state is a u32
        //let error = read_density_error(result.state as u32);
        panic!("Compression Error: {:?}", result.state);
    }
    
    let result: c_bindings::density_processing_result = decompress_block(
        &compressed_output, 
        result.bytes_written as _, 
        &mut decompressed_output,
        decompress_safe_size
    );

    if result.state == c_bindings::DENSITY_STATE_DENSITY_STATE_OK {
        println!("Decompressed {} bytes to {} bytes", result.bytesRead, result.bytesWritten);
        decompressed_output.truncate(result.bytesWritten as _);
    } else {
        let error = read_density_error(result.state);
        panic!("Decompression Error: {}", error);
    }
    
    //println!("text_bytes = {:?}\ndecompressed_output = {:?}", text_bytes, decompressed_output);
    assert!(text_bytes == &decompressed_output);
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