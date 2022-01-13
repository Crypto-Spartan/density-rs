use std::{
    alloc::{alloc_zeroed, Layout},
};

use crate::{
    alloc_boxed_array,
    algorithms, dictionary,
    algorithms::chameleon
};


pub fn chameleon_encode(in_buf: &[u8], out_buf: &mut [u8]) -> (algorithms::ExitStatus, usize, usize) {

    // calculate # whole chunks of 256 bytes in input buffer
    let num_chunks_256 = in_buf.len() >> 8;

    // verify that the output buffer is large enough
    if out_buf.len() < chameleon::CHAMELEON_MAX_COMPRESSED_UNIT_SIZE || (num_chunks_256+1) * 320 > out_buf.len() {
        //dbg!("DensityState::ErrorOutputBufferTooSmall", num_chunks_256, (num_chunks_256+1) * 320);
        //dbg!(out_buf.len(), chameleon::CHAMELEON_MAX_COMPRESSED_UNIT_SIZE);
        return (algorithms::ExitStatus::OutputStall, 0, 0);
    }
    
    // initialize iterators
    let in_chunks_256_iter = in_buf.chunks_exact(256);
    let in_chunks_256_remainder = in_chunks_256_iter.remainder();

    // initialize variables
    let mut chameleon_dictionary = alloc_boxed_array!(dictionary::CHAMELEON_DICT_SIZE);
    let mut copy_penalty: usize = 0;
    let mut copy_penalty_start: usize = 1;
    let mut previous_incompressible = false;

    let mut signature: u64 = 0;
    let mut signature_index: usize = 0;
    let mut out_index: usize = 0;

    // sequential loop - original DENSITY C library implementation, ported to Rust
    in_chunks_256_iter.enumerate().for_each(|(counter, chunk_256)| {

        // this is used to decrement copy_penalty_start periodically
        // `counter & 0xf == 0`: true every 16th iteration (e.g. 0, 16, 32...)
        if counter & 0xf == 0 && copy_penalty_start > 1 {
            copy_penalty_start >>= 1; // divide by 2, drop the remainder
        }

        // if we have a copy penalty, clone the entire chunk without ananlysis
        if copy_penalty > 0 {
            out_buf[out_index..out_index+256].clone_from_slice(chunk_256);
            out_index += 256;

            copy_penalty -= 1;
            // after the copy penalty is gone, increment copy_penalty_start
            // next time we get a copy penalty, it will be higher
            if copy_penalty == 0 {
                copy_penalty_start += 1;
            }

        // no copy penalty, perform hashing and compression
        } else {
            // this is where the actual compression happens
            signature = 0; // reset signature
            signature_index = out_index; // get index where signature will go in the output buffer
            out_index += 8;

            // from our 256 byte chunk, take 4 byte chunks
            chunk_256.chunks_exact(4).enumerate().for_each(|(shift, chunk_4)| {
                // turn the 4 byte chunk into a u32 and hash it
                let chunk_4_as_u32 = u32::from_le_bytes(chunk_4.try_into().unwrap());
                let hash: u16 = chameleon::chameleon_hash_function(chunk_4_as_u32);
                // check if we have seen the hash before
                let found_in_dict: u32 = chameleon_dictionary[hash as usize];

                // if we have seen the hash
                if chunk_4_as_u32 == found_in_dict {
                    // turn that bit on in the signature
                    signature |= 1u64 << shift;
                    // copy the hash to the output buffer
                    out_buf[out_index..out_index+2].clone_from_slice(&(hash.to_le_bytes()));
                    out_index += 2;

                // if we have not seen the hash
                } else {
                    // set the dictionary value for the hash
                    chameleon_dictionary[hash as usize] = chunk_4_as_u32;
                    // copy the raw 4 bytes to the output buffer (no compression on this 4 byte chunk)
                    out_buf[out_index..out_index+4].clone_from_slice(chunk_4);
                    out_index += 4;
                }
            });

            // write signature to output buffer
            out_buf[signature_index..signature_index+8].clone_from_slice(&(signature.to_le_bytes()));

            // if signature == 0, we weren't able to compress anything in this chunk
            if signature == 0 {
                // we only get a copy penalty if 2 chunks in a row are incompressible
                if previous_incompressible {
                    copy_penalty = copy_penalty_start;
                }
                previous_incompressible = true;
            } else {
                previous_incompressible = false;
            }
        }
    });

    //println!("in_chunks_256_remainder.len() = {}", in_chunks_256_remainder.len());

    signature = 0;
    signature_index = out_index;
    out_index += 8;

    let in_chunks_256_remainder_chunks_4 = in_chunks_256_remainder.chunks_exact(4);
    let remaining_bytes = in_chunks_256_remainder_chunks_4.remainder();

    in_chunks_256_remainder_chunks_4.enumerate().for_each(|(shift, chunk_4)| {
        let chunk_as_u32 = u32::from_le_bytes(chunk_4.try_into().unwrap());
        let hash: u16 = chameleon::chameleon_hash_function(chunk_as_u32);
        let found_in_dict: u32 = chameleon_dictionary[hash as usize];

        if chunk_as_u32 == found_in_dict {
            signature |= 1u64 << shift;
            out_buf[out_index..out_index+2].clone_from_slice(&(hash.to_le_bytes()));
            out_index += 2;
        } else {
            chameleon_dictionary[hash as usize] = chunk_as_u32;
            out_buf[out_index..out_index+4].clone_from_slice(chunk_4);
            out_index += 4;
        }
    });

    out_buf[signature_index..signature_index+8].clone_from_slice(&(signature.to_le_bytes()));


    let len_rem_bytes = remaining_bytes.len();
    // remaining_bytes.len() guarunteed to be 3 or less
    // if 0 bytes, nothing to do
    // if 1-3 bytes, copy them directly to the output buffer
    if len_rem_bytes > 0 {
        out_buf[out_index..out_index+len_rem_bytes].clone_from_slice(remaining_bytes);
        out_index += len_rem_bytes;
    }

    //println!("Bytes written: {}", out_index);

    (algorithms::ExitStatus::Finished, in_buf.len(), out_index)
}