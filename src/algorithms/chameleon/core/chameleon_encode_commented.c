/*
 * Density
 *
 * Copyright (c) 2013, Guillaume Voirin
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     1. Redistributions of source code must retain the above copyright notice, this
 *        list of conditions and the following disclaimer.
 *
 *     2. Redistributions in binary form must reproduce the above copyright notice,
 *        this list of conditions and the following disclaimer in the documentation
 *        and/or other materials provided with the distribution.
 *
 *     3. Neither the name of the copyright holder nor the names of its
 *        contributors may be used to endorse or promote products derived from
 *        this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * 23/06/15 22:02
 *
 * -------------------
 * Chameleon algorithm
 * -------------------
 *
 * Author(s)
 * Guillaume Voirin (https://github.com/gpnuma)
 *
 * Description
 * Hash based superfast kernel
 */

#include "chameleon_encode.h"

DENSITY_FORCE_INLINE void density_chameleon_encode_prepare_signature(uint8_t **DENSITY_RESTRICT out, density_chameleon_signature **DENSITY_RESTRICT signature_pointer, density_chameleon_signature *const DENSITY_RESTRICT signature) {
    *signature = 0;
    *signature_pointer = (density_chameleon_signature *) *out;
    *out += sizeof(density_chameleon_signature);
}

DENSITY_FORCE_INLINE void density_chameleon_encode_kernel(uint8_t **DENSITY_RESTRICT out, const uint16_t hash, const uint_fast8_t shift, density_chameleon_signature *const DENSITY_RESTRICT signature, density_chameleon_dictionary *const DENSITY_RESTRICT dictionary, uint32_t *DENSITY_RESTRICT unit) {
    // dictionary lookup
    density_chameleon_dictionary_entry *const found = &dictionary->entries[hash];

    // checking if they are equal
    switch (*unit ^ found->as_uint32_t) {
        // if they are equal (aka hash was found in dictionary)
        case 0:
            // add a 1 to the signature and shift the bits
            *signature |= ((uint64_t) DENSITY_CHAMELEON_SIGNATURE_FLAG_MAP << shift);
#ifdef DENSITY_LITTLE_ENDIAN
            // copy hash to output buffer
            DENSITY_MEMCPY(*out, &hash, sizeof(uint16_t));
#elif defined(DENSITY_BIG_ENDIAN)
            const uint16_t endian_hash = DENSITY_LITTLE_ENDIAN_16(hash);
            DENSITY_MEMCPY(*out, &endian_hash, sizeof(uint16_t));
#else
#error
#endif
            // increment pointer (essentially the index/length)
            *out += sizeof(uint16_t);
            break;

        // if switch statement not equal (aka hash not found in dict)
        default:
            // sets dictionary value
            found->as_uint32_t = *unit; // Does not ensure dictionary content consistency between endiannesses
            // copy raw data to output buffer
            DENSITY_MEMCPY(*out, unit, sizeof(uint32_t));
            // increment pointer (essentially the index/length)
            *out += sizeof(uint32_t);
            break;
    }
}

DENSITY_FORCE_INLINE void density_chameleon_encode_4(const uint8_t **DENSITY_RESTRICT in, uint8_t **DENSITY_RESTRICT out, const uint_fast8_t shift, density_chameleon_signature *const DENSITY_RESTRICT signature, density_chameleon_dictionary *const DENSITY_RESTRICT dictionary, uint32_t *DENSITY_RESTRICT unit) {
    // copy 4 bytes from input buffer to unit
    // unit becomes the variable we hash & analyze
    DENSITY_MEMCPY(unit, *in, sizeof(uint32_t));
    // do the dict lookup, hash comparison, encoding, etc
    density_chameleon_encode_kernel(out, DENSITY_CHAMELEON_HASH_ALGORITHM(DENSITY_LITTLE_ENDIAN_32(*unit)), shift, signature, dictionary, unit);
    // increment pointer (essentially the index/length)
    *in += sizeof(uint32_t);
}

DENSITY_FORCE_INLINE void density_chameleon_encode_256(const uint8_t **DENSITY_RESTRICT in, uint8_t **DENSITY_RESTRICT out, density_chameleon_signature *const DENSITY_RESTRICT signature, density_chameleon_dictionary *const DENSITY_RESTRICT dictionary, uint32_t *DENSITY_RESTRICT unit) {
    uint_fast8_t count = 0;

    // loop unrolling
#ifdef __clang__
    for (uint_fast8_t count_b = 0; count_b < 32; count_b++) {
        DENSITY_UNROLL_2(density_chameleon_encode_4(in, out, count++, signature, dictionary, unit));
    }
#else
    for (uint_fast8_t count_b = 0; count_b < 16; count_b++) {
        DENSITY_UNROLL_4(density_chameleon_encode_4(in, out, count++, signature, dictionary, unit));
    }
#endif
}


DENSITY_WINDOWS_EXPORT DENSITY_FORCE_INLINE density_algorithm_exit_status density_chameleon_encode(density_algorithm_state *const DENSITY_RESTRICT state, const uint8_t **DENSITY_RESTRICT in, const uint_fast64_t in_size, uint8_t **DENSITY_RESTRICT out, const uint_fast64_t out_size) {

    // check if output is big enough
    if (out_size < DENSITY_CHAMELEON_MAXIMUM_COMPRESSED_UNIT_SIZE) {
        return DENSITY_ALGORITHMS_EXIT_STATUS_OUTPUT_STALL;
    }

    // initialize variables
    // signature will be modified for every 256 bytes read from input_buffer
    density_chameleon_signature signature;
    // signature_pointer is the pointer to location in *out where the signature will be inserted
    density_chameleon_signature *signature_pointer;
    // 4 bytes that get hashed and compared to the dictionary
    uint32_t unit;

    // max pointer for output buffer
    uint8_t *out_limit = *out + out_size - DENSITY_CHAMELEON_MAXIMUM_COMPRESSED_UNIT_SIZE;
    // at least *this many* chunks of 256 bytes in input buffer
    uint_fast64_t limit_256 = (in_size >> 8);

    while (DENSITY_LIKELY(limit_256-- && *out <= out_limit)) {
        printf("------------------------------------------------------\nloop #: %d\n", state->counter+1);
        // true every 16th iteration
        if (DENSITY_UNLIKELY(!(state->counter & 0xf))) {
            printf("FLAG #1 - every 16th iteration\n");
            //printf("state->counter = %llu\n", state->counter);
            //printf("limit_256 = %llu\n", limit_256);
            //printf("out_limit - *out = %llu\n", out_limit - *out);
            printf("state->copy_penalty = %u\n", state->copy_penalty);
            printf("state->copy_penalty_start = %u\n", state->copy_penalty_start);
            printf("state->previous_incompressible = %s\n", state->previous_incompressible ? "true" : "false");
            //DENSITY_ALGORITHM_REDUCE_COPY_PENALTY_START;
            printf("state->copy_penalty_start & ~0x1 = %u\n", state->copy_penalty_start & ~0x1);
            if (state->copy_penalty_start & ~0x1) {
                printf("INCOMPRESSIBLE FLAG #4 - reduce copy_penalty_start\n");
                state->copy_penalty_start >>= 1;
            }
            printf("\n");
        }
        state->counter++;
        if (DENSITY_UNLIKELY(state->copy_penalty)) {
            printf("FLAG #2 - straight copy\n");
            printf("state->copy_penalty = %u\n", state->copy_penalty);
            printf("state->copy_penalty_start = %u\n", state->copy_penalty_start);
            printf("state->previous_incompressible = %s\n", state->previous_incompressible ? "true" : "false");
            // straight copy from input to output of 256 bytes
            DENSITY_ALGORITHM_COPY(DENSITY_CHAMELEON_WORK_BLOCK_SIZE);
            //DENSITY_ALGORITHM_INCREASE_COPY_PENALTY_START;
            if(!(--state->copy_penalty)) {
                printf("\nINCOMPRESSIBLE FLAG #5 - reduce copy_penalty; increase copy_penalty_start\n");
                state->copy_penalty_start++;
                printf("state->copy_penalty = %u\n", state->copy_penalty);
                printf("state->copy_penalty_start = %u\n", state->copy_penalty_start);
            }
            printf("\n");
        } else {
            const uint8_t *out_start = *out;
            // adds 8 to *out
            density_chameleon_encode_prepare_signature(out, &signature_pointer, &signature);
            DENSITY_PREFETCH(*in + DENSITY_CHAMELEON_WORK_BLOCK_SIZE);
            // *could* add up to 256 bytes to *out
            density_chameleon_encode_256(in, out, &signature, (density_chameleon_dictionary *const) state->dictionary, &unit);
            // at this comment, (*out - out_start) max is 256 + 8

            // ifdef copies
#ifdef DENSITY_LITTLE_ENDIAN
            DENSITY_MEMCPY(signature_pointer, &signature, sizeof(density_chameleon_signature));
#elif defined(DENSITY_BIG_ENDIAN)
            const density_chameleon_signature endian_signature = DENSITY_LITTLE_ENDIAN_64(signature);
            DENSITY_MEMCPY(signature_pointer, &endian_signature, sizeof(density_chameleon_signature));
#else
#error
#endif
            printf("bytes to out this loop: %u\n", (*out - out_start));
            // should always be 0 for u8 bits
            //printf("~(work_block_size - 1) = %hhu\n", ~(DENSITY_CHAMELEON_WORK_BLOCK_SIZE - 1));
            printf("over 256 bytes to out: %s\n", (*out - out_start) & ~(DENSITY_CHAMELEON_WORK_BLOCK_SIZE - 1) ? "true" : "false");
            //DENSITY_ALGORITHM_TEST_INCOMPRESSIBILITY((*out - out_start), DENSITY_CHAMELEON_WORK_BLOCK_SIZE);

             // if we wrote 256 or more bytes to output_buffer:
            if (DENSITY_UNLIKELY((*out - out_start) & ~(DENSITY_CHAMELEON_WORK_BLOCK_SIZE - 1))) {
                printf("INCOMPRESSIBLE FLAG #1 - wrote more than 256 bytes to out\n");
                if (signature == 0) {
                    printf("Signature == 0\n");
                }
                // if last iteration wrote 256+ bytes to output_buffer:
                if (state->previous_incompressible) {
                    printf("INCOMPRESSIBLE FLAG #2 - wrote 256+ bytes twice in a row - copy_penalty = copy_penalty_start\n");
                    state->copy_penalty = state->copy_penalty_start;
                    printf("copy_penalty and copy_penalty_start = %u\n", state->copy_penalty);
                }
                state->previous_incompressible = true;
                printf("state->previous_incompressible = %s\n", state->previous_incompressible ? "true" : "false");
            // else: (wrote 255 or less bytes to output_buffer)
            } else {
                printf("INCOMPRESSIBLE FLAG #3 - able to compress; wrote 255 bytes or less\n");
                state->previous_incompressible = false;
                //printf("state->previous_incompressible = %s\n", state->previous_incompressible ? "true" : "false");
            }

            printf("\n");
        }
    }
    // end while loop
    //printf("exited while loop\n");

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
}
