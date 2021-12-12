use density_rs::*; 
use std::mem;



fn main() {
    //test_linkage();
    //println!("UINT_LEAST32_MAX: {}", UINT_LEAST32_MAX);
    //let text = "This is a simple example on how to use the simple Density API.".as_bytes();
    //println!("{:?}", text);
    unsafe {
        round_trip_compression_decompression();
        /*let mut prev = 0;
        let mut diff;
        for i in (0..1_048_576).step_by(8192*2) {
            let buf = density_compress_safe_size(i as _);
            diff = buf - prev;
            println!("i={} buf={} diff={}", i, buf, diff);
            prev = buf;
        }*/
    }
}


unsafe fn round_trip_compression_decompression() {
    let text = "This is a simple example on how to use the".to_owned();// simple Density API....d.....a.........b.....z....e...qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmThis is a simple example on how to use the simple Density API....d.....a.........b.....z....e...qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmThis is a simple example on how to use the simple Density API....d.....a.........b.....z....e...qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmThis is a simple example on how to use the simple Density API....d.....a.........b.....z....e...qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmThis is a simple example on how to use the simple Density API....d.....a.........b.....z....e...qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnm".as_bytes();

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
    //let mut compressed_output_ptr = (&compressed_output).as_mut_ptr();
    //let mut decompressed_output_ptr = (&decompressed_output).as_mut_ptr();

    let result: density_processing_result = density_compress(
        text_bytes.as_ptr() as _, 
        text_length as _, 
        compressed_output.as_mut_ptr() as *mut _, 
        compress_safe_size as _, 
        DENSITY_ALGORITHM_DENSITY_ALGORITHM_CHAMELEON
    );

    println!("Rust FLAG #1");

    if result.state == DENSITY_STATE_DENSITY_STATE_OK {
        println!("Compressed {} bytes to {} bytes", result.bytesRead, result.bytesWritten);
    } else {
        let error = read_density_error(result.state);
        //result.state.f(); - u32
        panic!("Compression Error: {}", error);
    }

    println!("Rust FLAG #2");

    let result: density_processing_result = density_decompress(
        compressed_output.as_mut_ptr() as *mut _, 
        result.bytesWritten as _, 
        decompressed_output.as_mut_ptr() as *mut _,
        decompress_safe_size as _
    );

    println!("Rust FLAG #3");

    if result.state == DENSITY_STATE_DENSITY_STATE_OK {
        println!("Deompressed {} bytes to {} bytes", result.bytesRead, result.bytesWritten);
        decompressed_output.truncate(result.bytesWritten as _);
    } else {
        let error = read_density_error(result.state);
        panic!("Deompression Error: {}", error);
    }

    println!("Rust FLAG #4");
    println!("text_bytes = {:?}\ndecompressed_output = {:?}", text_bytes, decompressed_output);

    assert!(text_bytes == &decompressed_output);

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