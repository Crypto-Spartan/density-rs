use crate::c_bindings;
use std::ptr;

pub mod chameleon;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ExitStatus {
    Finished = 0,
    ErrorDuringProcessing = 1,
    InputStall = 2,
    OutputStall = 3
}


//#[derive(Clone, Debug)]
#[repr(C)]
pub struct State<'a> {
    dictionary: &'a mut [u32],
    copy_penalty: u8,
    copy_penalty_start: u8,
    previous_incompressible: bool,
    counter: u64
}

impl<'a> State<'a> {
    pub fn new(dict_slice: &'a mut [u32]) -> Self {
        State {
            dictionary: dict_slice,
            copy_penalty: 0,
            copy_penalty_start: 1,
            previous_incompressible: false,
            counter: 0
        }
    }

    pub fn to_c(self) -> c_bindings::density_algorithm_state {
        c_bindings::density_algorithm_state {
            dictionary: self.dictionary.as_mut_ptr() as *mut _,
            copy_penalty: self.copy_penalty as ::std::os::raw::c_uchar,
            copy_penalty_start: self.copy_penalty_start as ::std::os::raw::c_uchar,
            previous_incompressible: self.previous_incompressible,
            counter: self.counter as ::std::os::raw::c_ulong
        }
    }
}

pub fn reduce_copy_penalty_start(state: &mut State) {
    if state.copy_penalty_start & (!0x1) > 0 {
        state.copy_penalty_start >>= 1;
    }
}