use crate::{
    algorithms, algorithms::chameleon
};

pub fn chameleon_encode(state: &mut algorithms::State, in_buf: &[u8], out_buf: &mut [u8]) -> algorithms::ExitStatus {
    
    if out_size < chameleon::CHAMELEON_MAX_COMPRESSED_UNIT_SIZE as u64 {
        return algorithms::ExitStatus::OutputStall;
    }

    let out_limit = 

    let out_limit = unsafe {
        (*out_ptr as u64) + out_size - (chameleon::CHAMELEON_MAX_COMPRESSED_UNIT_SIZE as u64)
    };
    let limit_256: u64 = (in_size >> 8) - 1;

    unsafe {
        while limit_256 > 0 && *out_ptr as u64 <= out_limit {

            /*if state.counter & 0xf == 0 {
                algorithms::reduce_copy_penalty_start(&mut state);
            }*/
            
            state.counter += 1;

            if state.copy_penalty > 0 {
                algorithms::algorithm_copy(chameleon::CHAMELEON_WORK_BLOCK_SIZE, &mut in_ptr, &mut out_ptr);
            }
            
            limit_256 -= 1;
        }
    }
    

    algorithms::ExitStatus::Finished
}





/*density_algorithm_exit_status density_chameleon_encode(density_algorithm_state *const DENSITY_RESTRICT state, const uint8_t **DENSITY_RESTRICT in, const uint_fast64_t in_size, uint8_t **DENSITY_RESTRICT out, const uint_fast64_t out_size) {

    if (out_size < DENSITY_CHAMELEON_MAXIMUM_COMPRESSED_UNIT_SIZE) {
        return DENSITY_ALGORITHMS_EXIT_STATUS_OUTPUT_STALL;
    }

    density_chameleon_signature signature;
    density_chameleon_signature *signature_pointer;
    uint32_t unit;

    uint8_t *out_limit = *out + out_size - DENSITY_CHAMELEON_MAXIMUM_COMPRESSED_UNIT_SIZE;
    uint_fast64_t limit_256 = (in_size >> 8);

    while (DENSITY_LIKELY(limit_256-- && *out <= out_limit)) {
        if (DENSITY_UNLIKELY(!(state->counter & 0xf))) {
            DENSITY_ALGORITHM_REDUCE_COPY_PENALTY_START;
        }
        state->counter++;
        if (DENSITY_UNLIKELY(state->copy_penalty)) {
            DENSITY_ALGORITHM_COPY(DENSITY_CHAMELEON_WORK_BLOCK_SIZE);
            DENSITY_ALGORITHM_INCREASE_COPY_PENALTY_START;
        } else {
            const uint8_t *out_start = *out;
            density_chameleon_encode_prepare_signature(out, &signature_pointer, &signature);
            DENSITY_PREFETCH(*in + DENSITY_CHAMELEON_WORK_BLOCK_SIZE);
            density_chameleon_encode_256(in, out, &signature, (density_chameleon_dictionary *const) state->dictionary, &unit);
#ifdef DENSITY_LITTLE_ENDIAN
            DENSITY_MEMCPY(signature_pointer, &signature, sizeof(density_chameleon_signature));
#elif defined(DENSITY_BIG_ENDIAN)
            const density_chameleon_signature endian_signature = DENSITY_LITTLE_ENDIAN_64(signature);
            DENSITY_MEMCPY(signature_pointer, &endian_signature, sizeof(density_chameleon_signature));
#else
#error
#endif
            DENSITY_ALGORITHM_TEST_INCOMPRESSIBILITY((*out - out_start), DENSITY_CHAMELEON_WORK_BLOCK_SIZE);
        }
    }

    if (*out > out_limit)
        return DENSITY_ALGORITHMS_EXIT_STATUS_OUTPUT_STALL;

    uint_fast64_t remaining;

    switch (in_size & 0xff) {
        case 0:
        case 1:
        case 2:
        case 3:
            density_chameleon_encode_prepare_signature(out, &signature_pointer, &signature);
            signature = ((uint64_t) DENSITY_CHAMELEON_SIGNATURE_FLAG_CHUNK);    // End marker
#ifdef DENSITY_LITTLE_ENDIAN
            DENSITY_MEMCPY(signature_pointer, &signature, sizeof(density_chameleon_signature));
#elif defined(DENSITY_BIG_ENDIAN)
            const density_chameleon_signature endian_signature = DENSITY_LITTLE_ENDIAN_64(signature);
            DENSITY_MEMCPY(signature_pointer, &endian_signature, sizeof(density_chameleon_signature));
#else
#error
#endif
            goto process_remaining_bytes;
        default:
            break;
    }

    const uint_fast64_t limit_4 = (in_size & 0xff) >> 2;
    density_chameleon_encode_prepare_signature(out, &signature_pointer, &signature);
    for (uint_fast8_t shift = 0; shift != limit_4; shift++)
        density_chameleon_encode_4(in, out, shift, &signature, (density_chameleon_dictionary *const) state->dictionary, &unit);

    signature |= ((uint64_t) DENSITY_CHAMELEON_SIGNATURE_FLAG_CHUNK << limit_4);    // End marker
#ifdef DENSITY_LITTLE_ENDIAN
    DENSITY_MEMCPY(signature_pointer, &signature, sizeof(density_chameleon_signature));
#elif defined(DENSITY_BIG_ENDIAN)
    const density_chameleon_signature endian_signature = DENSITY_LITTLE_ENDIAN_64(signature);
    DENSITY_MEMCPY(signature_pointer, &endian_signature, sizeof(density_chameleon_signature));
#else
#error
#endif

    process_remaining_bytes:
    remaining = in_size & 0x3;
    if (remaining)
        DENSITY_ALGORITHM_COPY(remaining);

    return DENSITY_ALGORITHMS_EXIT_STATUS_FINISHED;
}*/