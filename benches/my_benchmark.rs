use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use density_rs::{DensityAlgorithm, DensityResult, compress_block};

fn bench_compression(c: &mut Criterion) {
    
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
    //println!("compress_safe_size = {}", compress_safe_size);

    let mut compressed_output = vec![0u8; compress_safe_size as usize];

    /*let result: DensityResult = compress_block(
        &text_bytes, 
        &mut compressed_output,
        DensityAlgorithm::Chameleon
    );*/

    c.bench_function("Chameleon Compression New", 
        |b| b.iter(|| {
            compress_block(&text_bytes, &mut compressed_output, DensityAlgorithm::Chameleon)
        })
    );

}

criterion_group!(benches, bench_compression);
criterion_main!(benches);

/*fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);*/