#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
pub mod utils;

#[macro_use]
pub mod globals;

#[cfg(test)]
pub mod tests;

pub mod c_bindings;
pub mod header;
pub mod buffer;
pub mod algorithms;

mod dictionary;
pub use dictionary::DictType;


#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum DensityAlgorithm {
    Chameleon = 1,
    Cheetah = 2,
    Lion = 3
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum DensityState {
    OK = 0,                                   // Everything went alright
    ErrorInputBufferTooSmall = 1,             // Input buffer size is too small
    ErrorOutputBufferTooSmall = 2,            // Output buffer size is too small
    ErrorDuringProcessing = 3,                // Error during processing
    ErrorInvalidContext = 4,                  // Invalid context
    ErrorInvalidAlgorithm = 5,                // Invalid algorithm    
}

/*
pub struct DensityContext {
    algorithm: DensityAlgorithm,
    dict_size: usize,
    dictionary: Box<[u32; 65_535]> // or vec? but then bounds checks
    // https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new_zeroed
}*/

pub struct DensityResult {
    pub state: DensityState,
    pub bytes_read: usize,
    pub bytes_written: usize
}

pub enum ResultEnum {
    Rust(DensityResult),
    C(c_bindings::density_processing_result)
}


pub fn compress_block(input_buffer: &[u8], output_buffer: &mut [u8], algorithm: DensityAlgorithm) -> DensityResult {
    buffer::compress_with_dict(input_buffer, output_buffer, algorithm)
}


pub fn decompress_block(
    input_buffer: &[u8], 
    input_size: u64, 
    output_buffer: &mut [u8], 
    output_size: u64
) -> c_bindings::density_processing_result {

    let input_buffer_ptr = input_buffer.as_ptr() as _;
    let output_buffer_ptr = output_buffer.as_mut_ptr() as *mut _;
    let mut result: c_bindings::density_processing_result; 
    
    unsafe {
        result = c_bindings::density_decompress_prepare_context(input_buffer_ptr, input_size, false);
        if result.state != 0u32 {
            c_bindings::density_free_context(result.context);
            return result;
        }

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