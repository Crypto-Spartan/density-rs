use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, BatchSize};
use rand::prelude::*;
use density_rs::{
    c_bindings,
    DensityResult, DensityState, DensityAlgorithm,
    compress_block//, decompress_block
};


fn bench_rust_compression(c: &mut Criterion) {
    
    //let text = "This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+".to_owned();
    
    let data_len: usize = 1_048_576;
    let mut text = vec![0u8; data_len];
    let mut seeded_rng = StdRng::seed_from_u64(510152025);

    for i in text.iter_mut() {
        *i = seeded_rng.gen();
    }

    let text_bytes = &text[..];
    let text_length = text_bytes.len();

    let num_chunks_256 = text_length >> 8;
    let compress_safe_size = (num_chunks_256+1) * 320;

    let mut compressed_output = vec![0u8; compress_safe_size as usize];

    c.bench_function("Chameleon Compression New", 
        |b| b.iter(|| {
            compress_block(&text_bytes, &mut compressed_output, DensityAlgorithm::Chameleon)
        })
    );

}


struct RustAndCBenchInput {
    text: Vec<u8>,
    compressed_output: Vec<u8>
}

fn do_c_compression(input: &mut RustAndCBenchInput) {
    let text_bytes = &input.text;
    let mut compressed_output = &mut input.compressed_output;

    unsafe {
        let result: c_bindings::density_processing_result = c_bindings::density_compress(
            text_bytes.as_ptr() as _,
            text_bytes.len() as _,
            compressed_output.as_mut_ptr() as *mut _,
            compressed_output.len() as _,
            c_bindings::DENSITY_ALGORITHM_DENSITY_ALGORITHM_CHAMELEON
        );

        if result.state == c_bindings::DENSITY_STATE_DENSITY_STATE_OK {
            compressed_output.truncate(result.bytesWritten as _);
        } else {
            //result.state is a u32
            let error = match result.state {
                1u32 => "DENSITY_STATE_ERROR_INPUT_BUFFER_TOO_SMALL".to_owned(),
                2u32 => "DENSITY_STATE_ERROR_OUTPUT_BUFFER_TOO_SMALL".to_owned(),
                3u32 => "DENSITY_STATE_ERROR_DURING_PROCESSING".to_owned(),
                4u32 => "DENSITY_STATE_ERROR_INVALID_CONTEXT".to_owned(),
                5u32 => "DENSITY_STATE_ERROR_INVALID_ALGORITHM".to_owned(),
                _ => unreachable!()
            };
            panic!("C: Compression Error: {}", error);
        }
    }
}

fn do_rust_compression(input: &mut RustAndCBenchInput) {
    let text_bytes = &input.text;
    let mut compressed_output = &mut input.compressed_output;

    let result: DensityResult = compress_block(
        text_bytes,
        compressed_output,
        DensityAlgorithm::Chameleon
    );

    if matches!(result.state, DensityState::OK) {
        compressed_output.truncate(result.bytes_written);
    } else {
        panic!("Rust: Compression Error: {:?}", result.state);
    }
}

fn bench_rust_and_c_chameleon(c: &mut Criterion) {
    let text = "This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ This is a simple example on how to use the simple Density API. (Here's some data to make this string longer) qwertyuiop[]asdfghjkl;:zxcvbnm,<.>/?`~1!2@3#4$5%6^7&8*9(0)-_=+ ".to_owned();
    
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();
    let num_chunks_256 = text_length >> 8;

    let compress_safe_size_c: u64;
    unsafe {
        compress_safe_size_c = c_bindings::density_compress_safe_size(text_length as _);
    }
    let compress_safe_size_rust = ((num_chunks_256+1) * 320) + 8;

    let mut compressed_output_c = vec![0u8; compress_safe_size_c as usize];
    let mut compressed_output_rust = vec![0u8; compress_safe_size_rust as usize];
    
    let mut bench_group = c.benchmark_group("Chameleon");

    bench_group.bench_function("Rust", |b| {
            b.iter_batched_ref(
                || { 
                    RustAndCBenchInput{
                        text: text_bytes.to_vec(),
                        compressed_output: compressed_output_rust.clone()
                    }
                }, 
                |mut output| do_rust_compression(&mut output), 
                BatchSize::SmallInput
            )
        }
    );

    bench_group.bench_function("C", |b| {
            b.iter_batched_ref(
                || {
                    RustAndCBenchInput{
                        text: text_bytes.to_vec(),
                        compressed_output: compressed_output_c.clone()
                    }
                }, 
                |mut output| do_c_compression(&mut output), 
                BatchSize::SmallInput
            )
        }
    );

    bench_group.finish();
}


//criterion_group!(benches, bench_rust_compression);
criterion_group!(benches, bench_rust_and_c_chameleon);
criterion_main!(benches);