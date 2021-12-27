use crate::DensityAlgorithm;
use crate::globals::{MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION};

pub const sizeof_DensityHeader: usize = 8;


//#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[derive(Debug)]
pub struct DensityHeader {
    version: [u8; 3],
    algorithm: u8,
    reserved: [u8; 4]
}


#[inline(always)]
pub fn read(input_buffer: &[u8], header: &mut DensityHeader) {
    header.version.clone_from_slice(&input_buffer[..3]);
    header.algorithm = input_buffer[3];
}

#[inline(always)]
pub fn write(output_buffer: &mut [u8], algorithm: DensityAlgorithm) {
    let tmp = [MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION, algorithm as u8, 0u8, 0u8, 0u8, 0u8];
    //println!("hex of output_buffer: {:x}", u64::from_ne_bytes(tmp.clone()));
    output_buffer[..8].clone_from_slice(&tmp);
}

/*
b buffer.rs:28
b chameleon_encode.c:84

(gdb) p *out_ptr
$5 = (*mut u8) 0x1020e00
*/