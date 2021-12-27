pub mod encode;


pub const CHAMELEON_HASH_BITS: usize = 16;
pub const CHAMELEON_HASH_MULTIPLIER: u32 = 0x9D6EF916;


pub const CHAMELEON_MAX_COMPRESSED_BODY_SIZE_PER_SIGNATURE: usize = 256;
//#define DENSITY_CHAMELEON_MAXIMUM_COMPRESSED_BODY_SIZE_PER_SIGNATURE        (density_bitsizeof(density_chameleon_signature) * sizeof(uint32_t))   // Uncompressed chunks
pub const CHAMELEON_DECOMPRESSED_BODY_SIZE_PER_SIGNATURE: usize = 256;
//#define DENSITY_CHAMELEON_DECOMPRESSED_BODY_SIZE_PER_SIGNATURE              (density_bitsizeof(density_chameleon_signature) * sizeof(uint32_t))
pub const CHAMELEON_MAX_COMPRESSED_UNIT_SIZE: usize = 320;
//#define DENSITY_CHAMELEON_MAXIMUM_COMPRESSED_UNIT_SIZE                      (sizeof(density_chameleon_signature) + DENSITY_CHAMELEON_MAXIMUM_COMPRESSED_BODY_SIZE_PER_SIGNATURE)
pub const CHAMELEON_DECOMPRESSED_UNIT_SIZE: usize = 256;
//#define DENSITY_CHAMELEON_DECOMPRESSED_UNIT_SIZE                            (DENSITY_CHAMELEON_DECOMPRESSED_BODY_SIZE_PER_SIGNATURE)
pub const CHAMELEON_WORK_BLOCK_SIZE: usize = 256;
//#define DENSITY_CHAMELEON_WORK_BLOCK_SIZE                                   256


#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ChameleonSignatureFlag {
    Chunk = 0,
    Map = 1
}


pub fn chameleon_hash_function(value: u32) -> u16 {
    // right shift: (32 - DENSITY_CHAMELEON_HASH_BITS) = 16
    ((value * CHAMELEON_HASH_MULTIPLIER) >> 16) as u16
}