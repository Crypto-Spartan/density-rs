use std::{
    //mem,
    alloc::{alloc_zeroed, Layout},
    ptr
};
use crate::{
    header, c_bindings,
    algorithms,//::{State, ExitStatus},
    dictionary,
    DensityAlgorithm, DensityState, /*DensityContext,*/ DensityResult, DictType
};


macro_rules! alloc_boxed_array {
    ($x: expr) => {{
        let layout = Layout::new::<[u32; $x]>();
        unsafe { 
            let ptr = alloc_zeroed(layout) as *mut [u32; $x];
            Box::from_raw(ptr)
        }
    }}
}

/*
pub fn compress_prepare_dict(algorithm: DensityAlgorithm) -> DensityResult {
    make_result(DensityState::OK, 0, 0)//, DictType::new(algorithm))
}*/

pub fn compress_with_dict_c(input_buffer: &[u8], input_size: u64, output_buffer: &mut [u8], output_size: u64,
algorithm: DensityAlgorithm)/*, mut dict_type: DictType)*/ -> DensityResult {
    
    // sizeof(density_header) = 8 bytes
    if output_size < (header::sizeof_DensityHeader as u64) {
        return make_result(DensityState::ErrorOutputBufferTooSmall, 0, 0);
    }

    let (out_header, out_data) = output_buffer.split_at_mut(8);
    header::write(out_header, algorithm);

    match algorithm {
        DensityAlgorithm::Chameleon => {
            let mut boxed_array = alloc_boxed_array!(dictionary::CHAMELEON_DICT_SIZE);
            
            let mut state = algorithms::State::new(&mut boxed_array[..]);

            let status = algorithms::chameleon::chameleon_encode(
                &mut state,
                &input_buffer,
                &mut output_buffer
            );

            // just so we have a fake return while coding
            DensityResult::OK

            // this works with c bindings 
            /*let mut state = algorithms::State::new(&mut boxed_array[..]).to_c();
            let mut in_ptr = input_buffer.as_ptr() as *const _;
            let mut out_ptr = out_data.as_mut_ptr() as *mut _;

            let status = unsafe {
                c_bindings::density_chameleon_encode(
                    ptr::addr_of_mut!(state),
                    ptr::addr_of_mut!(in_ptr),
                    input_size, 
                    ptr::addr_of_mut!(out_ptr),
                    output_size
                )
            };

            make_result(
                convert_algorithm_exit_status_from_c(status as u32), 
                in_ptr as u64 - input_buffer.as_ptr() as u64,
                out_ptr as u64 - output_buffer.as_ptr() as u64
            )*/
        },
        _ => {
            todo!();
        }
    }

}

/*density_processing_result density_compress_with_context(const uint8_t * input_buffer, const uint_fast64_t input_size, uint8_t * output_buffer, const uint_fast64_t output_size, density_context *const context) {

    // sizeof(density_header) = 8 bytes
    if (output_size < sizeof(density_header)) {
        return density_make_result(DENSITY_STATE_ERROR_OUTPUT_BUFFER_TOO_SMALL, 0, 0, context);
    }
    if(context == NULL) {
        return density_make_result(DENSITY_STATE_ERROR_INVALID_CONTEXT, 0, 0, context);
    }

    // Variables setup
    const uint8_t *in = input_buffer;
    uint8_t *out = output_buffer;
    density_algorithm_state state;
    density_algorithm_exit_status status = DENSITY_ALGORITHMS_EXIT_STATUS_ERROR_DURING_PROCESSING;

    // Header
    density_header_write(&out, context->algorithm);

    // Compression
    density_algorithms_prepare_state(&state, context->dictionary);
    switch (context->algorithm) {
        case DENSITY_ALGORITHM_CHAMELEON:
            status = density_chameleon_encode(&state, &in, input_size, &out, output_size);
            break;
        case DENSITY_ALGORITHM_CHEETAH:
            status = density_cheetah_encode(&state, &in, input_size, &out, output_size);
            break;
        case DENSITY_ALGORITHM_LION:
            status = density_lion_encode(&state, &in, input_size, &out, output_size);
            break;
    }

    // Result
    return density_make_result(density_convert_algorithm_exit_status(status), in - input_buffer, out - output_buffer, context);
}*/



pub fn make_result(
    state: DensityState,
    bytes_read: u64, 
    bytes_written: u64
) -> DensityResult {
    /*println!("fn make_result()");
    println!("state = {:?}", state);
    println!("bytes_read = {}", bytes_read);
    println!("bytes_written = {}", bytes_written);*/
    //println!("dict = {}", dict);
    DensityResult {
        state,
        bytes_read,
        bytes_written
    }
}

/*pub fn allocate_context(algorithm: DensityAlgorithm) -> DensityContext {

    // fastest way to allocate an array on the heap in rust, only 3 asm lines
    // https://godbolt.org/z/sqvMrez91
    // box array stack overflow issue: https://github.com/rust-lang/rust/issues/53827
    // rust will also handle deallocation automatically
    let mut dictionary = {
        let layout = Layout::new::<[u32; 65_535]>();
        let buf = unsafe {
            let ptr = alloc_zeroed(layout) as *mut [u32; 65_535];
            Box::from_raw(ptr)
        };
        buf
    };
    let dict_size = get_dictionary_size(algorithm);

    DensityContext {
        algorithm,
        dict_size,
        dictionary
    }
}*/


pub fn convert_algorithm_exit_status(algo_status: algorithms::ExitStatus) -> DensityState {
    match algo_status {
        algorithms::ExitStatus::Finished => {
            DensityState::OK
        },
        algorithms::ExitStatus::InputStall => {
            DensityState::ErrorInputBufferTooSmall
        },
        algorithms::ExitStatus::OutputStall => {
            DensityState::ErrorOutputBufferTooSmall
        },
        _ => {
            DensityState::ErrorDuringProcessing
        }
    }
}

pub fn convert_algorithm_exit_status_from_c(algo_status: u32) -> DensityState {
    match algo_status {
        0 => {
            DensityState::OK
        },
        1 => {
            DensityState::ErrorInputBufferTooSmall
        },
        2 => {
            DensityState::ErrorOutputBufferTooSmall
        },
        _ => {
            DensityState::ErrorDuringProcessing
        }
    }
}