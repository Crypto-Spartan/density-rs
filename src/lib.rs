#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
pub mod utils;

#[cfg(test)]
pub mod tests;

pub mod c_bindings;


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



pub fn decompress(
    input_buffer: &[u8], 
    input_size: u64, 
    output_buffer: &mut [u8], 
    output_size: u64
) -> c_bindings::density_processing_result {

    let input_buffer_ptr = input_buffer.as_ptr() as _;
    let output_buffer_ptr = output_buffer.as_mut_ptr() as *mut _;
    
    let mut result: c_bindings::density_processing_result = unsafe {
        c_bindings::density_decompress_prepare_context(input_buffer_ptr, input_size, false)
    };
    if result.state > 0u32 {
        unsafe { c_bindings::density_free_context(result.context); }
        return result;
    }

    unsafe {
        result = c_bindings::density_decompress_with_context(
            input_buffer_ptr.offset(result.bytesRead.try_into().unwrap()), 
            input_size - result.bytesRead, 
            output_buffer_ptr, 
            output_size, 
            result.context
        );
        c_bindings::density_free_context(result.context);
    }
    
    return result;
}