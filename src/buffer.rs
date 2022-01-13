#![allow(unused_imports)]

use std::{
    alloc::{alloc_zeroed, Layout},
};
use crate::{
    alloc_boxed_array,
    header, c_bindings,
    algorithms,//::{State, ExitStatus},
    dictionary,
    DensityAlgorithm, DensityState, /*DensityContext,*/ DensityResult, DictType
};


pub fn compress_with_dict(input_buffer: &[u8], output_buffer: &mut [u8], algorithm: DensityAlgorithm) -> DensityResult {
    
    // sizeof(density_header) = 8 bytes
    if output_buffer.len() < header::sizeof_DensityHeader {
        //dbg!("DensityState::ErrorOutputBufferTooSmall");
        return make_result(DensityState::ErrorOutputBufferTooSmall, 0, 0);
    }

    let (out_header, out_data) = output_buffer.split_at_mut(8);
    header::write(out_header, algorithm);

    match algorithm {
        DensityAlgorithm::Chameleon => {
            //let mut boxed_array = alloc_boxed_array!(dictionary::CHAMELEON_DICT_SIZE);
            
            //let mut out_index: usize = 0;
            //let mut state = algorithms::State::new(&mut boxed_array[..]);

            let status = algorithms::chameleon::encode::chameleon_encode(
                //&mut state,
                input_buffer,
                out_data
            );

            /*// just so we have a fake return while coding
            make_result(DensityState::OK, input_buffer.len(), out_index)*/

            match status.0 {
                algorithms::ExitStatus::Finished => {
                    make_result(DensityState::OK, status.1, status.2)
                },
                algorithms::ExitStatus::OutputStall => {
                    make_result(DensityState::ErrorOutputBufferTooSmall, status.1, status.2)
                }
                _ => {
                    make_result(DensityState::ErrorDuringProcessing, status.1, status.2)
                }
            }
        },
        _ => {
            //todo!();
            make_result(DensityState::ErrorInvalidAlgorithm, 0, 0)
        }
    }
}


pub fn make_result(
    state: DensityState,
    bytes_read: usize, 
    bytes_written: usize
) -> DensityResult {
    DensityResult {
        state,
        bytes_read,
        bytes_written
    }
}


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