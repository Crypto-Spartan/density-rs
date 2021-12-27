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

pub fn algorithm_copy(work_block_size: usize, in_ptr: &mut *mut *const u8, out_ptr: &mut *mut *mut u8) {
    in_ptr.f();
    (*in_ptr).f();
    (**in_ptr).f();
    (&(**in_ptr)).f();
    unsafe {
        //ptr::copy_nonoverlapping(*in_ptr, *out_ptr, work_block_size);
        *in_ptr = *in_ptr.offset(work_block_size as isize);
        *out_ptr = *out_ptr.offset(work_block_size as isize);
    }
}

/*
#define DENSITY_ALGORITHM_COPY(work_block_size)\
            DENSITY_MEMCPY(*out, *in, work_block_size);\
            *in += work_block_size;\
            *out += work_block_size;*/