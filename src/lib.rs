#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
pub mod utils;

pub mod c_bindings;
pub mod tests;


//density_processing_result density_compress(const uint8_t *input_buffer, const uint_fast64_t input_size, uint8_t *output_buffer, const uint_fast64_t output_size, const DENSITY_ALGORITHM algorithm) 

pub fn compress(
    input_buffer: &[u8], 
    input_size: u64, 
    output_buffer: &mut [u8], 
    output_size: u64, 
    algorithm: c_bindings::DENSITY_ALGORITHM
) -> c_bindings::density_processing_result {
    let mut result: c_bindings::density_processing_result = unsafe {
        c_bindings::density_compress_prepare_context(algorithm, false)
    };
    if result.state > 0u32 {
        unsafe { c_bindings::density_free_context(result.context); }
        return result;
    }

    unsafe {
        result = c_bindings::density_compress_with_context(
            input_buffer.as_ptr() as _, 
            input_size, 
            output_buffer.as_mut_ptr() as *mut _, 
            output_size, 
            result.context
        );
        c_bindings::density_free_context(result.context);
    };
    return result;
}

/*density_processing_result density_decompress(const uint8_t *input_buffer, const uint_fast64_t input_size, uint8_t *output_buffer, const uint_fast64_t output_size) 

pub fn decompress(
    input_buffer: &[u8], 
    input_size: u64, 
    output_buffer: &mut [u8], 
    output_size: u64
) -> c_bindings::density_processing_result {
    
    density_processing_result result = density_decompress_prepare_context(input_buffer, input_size, false, malloc);
    if(result.state) {
        density_free_context(result.context);
        return result;
    }

    result = density_decompress_with_context(input_buffer + result.bytesRead, input_size - result.bytesRead, output_buffer, output_size, result.context);
    density_free_context(result.context);
    return result;
*/
/*

DENSITY_WINDOWS_EXPORT density_processing_result density_decompress(const uint8_t *input_buffer, const uint_fast64_t input_size, uint8_t *output_buffer, const uint_fast64_t output_size) {
    density_processing_result result = density_decompress_prepare_context(input_buffer, input_size, false, malloc);
    if(result.state) {
        density_free_context(result.context);
        return result;
    }

    result = density_decompress_with_context(input_buffer + result.bytesRead, input_size - result.bytesRead, output_buffer, output_size, result.context);
    density_free_context(result.context);
    return result;
}*/