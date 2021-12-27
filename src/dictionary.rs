use std::alloc::{Layout, alloc_zeroed};
use crate::DensityAlgorithm;


// all sizes in bytes
pub const CHAMELEON_DICT_SIZE: usize = 262_144;     // 4 bytes (u32) * 65_536
pub const CHEETAH_DICT_SIZE: usize = 786_432;       // ((4 bytes (u32) * 2) * 65_536) + (4 bytes (u32) * 65_536)
pub const LION_DICT_SIZE: usize = 2_097_152;        // ((4 bytes (u32) * 5) * 65_536) + ((4 bytes (u32) * 3) * 65_536)

// same number values as DensityAlgorithm
pub enum DictType {
    Chameleon(Box<[u32; CHAMELEON_DICT_SIZE]>), //= 1,
    Cheetah(Box<[u32; CHEETAH_DICT_SIZE]>), //= 2,
    Lion(Box<[u32; LION_DICT_SIZE]>) //= 3
}


/*let layout = Layout::new::<[u32; CHAMELEON_DICT_SIZE]>();
let buf = unsafe { 
    let ptr = alloc_zeroed(layout) as *mut [u32; CHAMELEON_DICT_SIZE];
    Box::from_raw(ptr)
}*/
macro_rules! alloc_boxed_array {
    ($x: expr) => {{
        let layout = Layout::new::<[u32; $x]>();
        unsafe { 
            let ptr = alloc_zeroed(layout) as *mut [u32; $x];
            Box::from_raw(ptr)
        }
    }}
}

impl DictType {
    pub fn new(algorithm: DensityAlgorithm) -> Self {
        match algorithm {
            DensityAlgorithm::Chameleon => {
                let buf = alloc_boxed_array!(CHAMELEON_DICT_SIZE);
                DictType::Chameleon(buf)
            },
            DensityAlgorithm::Cheetah => {
                let buf = alloc_boxed_array!(CHEETAH_DICT_SIZE);
                DictType::Cheetah(buf)
            },
            DensityAlgorithm::Lion => {
                let buf = alloc_boxed_array!(LION_DICT_SIZE);
                DictType::Lion(buf)
            }
        }
    }
}

/*fn make_dictionary(const dict_size: usize) -> [u32; dict_size] {
    let layout = Layout::new::<[u32; dict_size]>();
    unsafe {
        let ptr = alloc_zeroed(layout) as *mut [u32; dict_size];
        Box::from_raw(ptr)
    }
}*/

/*size_t density_get_dictionary_size(DENSITY_ALGORITHM algorithm) {
    switch(algorithm) {
        case DENSITY_ALGORITHM_CHAMELEON:
            return sizeof(density_chameleon_dictionary);
        case DENSITY_ALGORITHM_CHEETAH:
            return sizeof(density_cheetah_dictionary);
        case DENSITY_ALGORITHM_LION:
            return sizeof(density_lion_dictionary);
        default:
            return 0;
    }
}*/