use crate::{
    algorithms, algorithms::chameleon
};
use rayon::prelude::*;

#[derive(Default, Debug)]
pub struct Chunk4Bytes<'a> {
    slice: &'a [u8],
    as_u32: u32,
    hash: u16
}

//pub fn chameleon_encode_kernel()

pub fn chameleon_encode(state: &mut algorithms::State, in_buf: &[u8], out_buf: &mut [u8]) -> algorithms::ExitStatus {
    
    let num_chunks_256 = in_buf.len() >> 8;
    //let remainder_bool = in_buf.len() & 0x80;
    if out_buf.len() < chameleon::CHAMELEON_MAX_COMPRESSED_UNIT_SIZE || (num_chunks_256+1) * 320 > out_buf.len() {
        return algorithms::ExitStatus::OutputStall;
    }
    
    //let out_limit: usize = (out_buf.len() - chameleon::CHAMELEON_MAX_COMPRESSED_UNIT_SIZE) as usize;
    let in_chunks_4_iter = in_buf.par_chunks_exact(4);
    let in_chunks_4_remainder = in_chunks_4_iter.remainder();
    
    let mut chunks4_vec: Vec<Chunk4Bytes> = Vec::with_capacity(in_chunks_4_iter.len());

    chunks4_vec.par_extend(
        in_chunks_4_iter.map(|chunk_4| {
            let chunk_as_u32 = u32::from_le_bytes(chunk_4.try_into().unwrap());
            let hash: u16 = chameleon::chameleon_hash_function(chunk_as_u32);
            
            Chunk4Bytes{
                slice: chunk_4,
                as_u32: chunk_as_u32,
                hash
            }
        })
    );

    let mut out_index: usize = 8;

    let chunks4_vec_64 = chunks4_vec.chunks_exact(64);
    let chunks4_vec_remaining = chunks4_vec_64.remainder();

    chunks4_vec_64.for_each(|chunk_64| {
        chameleon_encode_256(chunk_64, state.dictionary, out_buf, &mut out_index);
    });

    // checks if there is remaining data in in_buf
    match in_buf.len() & 0xff {
        0 => (),
        rem_bytes @ 1..=3 => {
            out_buf[out_index..out_index+rem_bytes].clone_from_slice(&in_buf[in_buf.len()-rem_bytes..]);
            out_index += rem_bytes;
        },
        rem_bytes => {
            chameleon_encode_256(chunks4_vec_remaining, state.dictionary, out_buf, &mut out_index);

            if in_chunks_4_remainder.len() > 0 {
                out_buf[out_index..out_index+in_chunks_4_remainder.len()]
                    .clone_from_slice(&in_buf[in_buf.len()-in_chunks_4_remainder.len()..]);
                out_index += rem_bytes;
            }
        }
    }

    println!("Bytes written: {}", out_index);
    

    algorithms::ExitStatus::Finished
}





pub fn chameleon_encode_256(chunk_slice: &[Chunk4Bytes], dictionary: &mut [u32], out_buf: &mut [u8], out_index: &mut usize) {
    let mut signature = 0;
    let signature_index = *out_index;
    *out_index += 8;

    chunk_slice.iter().enumerate().for_each(|(shift, chunk_4)| {
        let found_in_dict: u32 = dictionary[chunk_4.hash as usize];
        
        if chunk_4.as_u32 == found_in_dict {
            dictionary[chunk_4.hash as usize] = chunk_4.as_u32;
            out_buf[*out_index..*out_index+4].clone_from_slice(chunk_4.slice);
            *out_index += 4;
        } else if found_in_dict == 0 {
            signature |= (chameleon::ChameleonSignatureFlag::Map as u64) << shift;
            out_buf[*out_index..*out_index+2].clone_from_slice(&(chunk_4.hash.to_le_bytes()));
            *out_index += 2;
        } else {
            println!("COLLISION: chunk_4 = {:x} | found_in_dict = {:x}", chunk_4.as_u32, found_in_dict);
            dictionary[chunk_4.hash as usize] = chunk_4.as_u32;
            out_buf[*out_index..*out_index+4].clone_from_slice(chunk_4.slice);
            *out_index += 4;
        }
    });
    out_buf[signature_index..signature_index+8].clone_from_slice(&(signature.to_le_bytes()));
}