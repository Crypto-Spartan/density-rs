#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
pub mod utils;

pub mod c_bindings;
pub mod tests;


/*DENSITY_WINDOWS_EXPORT density_processing_result density_compress(const uint8_t *input_buffer, const uint_fast64_t input_size, uint8_t *output_buffer, const uint_fast64_t output_size, const DENSITY_ALGORITHM algorithm) {
    density_processing_result result = density_compress_prepare_context(algorithm, false, malloc);
    if(result.state) {
        density_free_context(result.context);
        return result;
    }

    result = density_compress_with_context(input_buffer, input_size, output_buffer, output_size, result.context);
    density_free_context(result.context);
    return result;
}

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