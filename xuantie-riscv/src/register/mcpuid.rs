//! mcpuid, processor identification register
//!
//! # Platform support
//!
//! This register is supported on Xuantie C910, C906, E907, E906 and E902 cores.
//!
//! todo: verify mcpuid read and look it up on C906 manual
use bit_field::BitField;
use core::arch::asm;

/// Processor information
#[derive(Clone, Copy, Debug)]
pub struct Cpuid {
    #[allow(unused)] // todo: remove when all fields implemented
    data: [u32; 7],
}

// todo: all the fields
impl Cpuid {
    // pub fn arch(self) -> usize {

    // }
}

/// Fetch the processor information
#[inline]
pub fn read() -> Cpuid {
    let mut data = [0; 7];
    for _ in 0..7 {
        let val: u32;
        unsafe { asm!("csrr {}, 0xFC0", out(reg) val) };
        match val.get_bits(28..=31) {
            idx @ 0..=6 => data[idx as usize] = val,
            _ => {} // ignore invalid data indexes
        }
    }
    Cpuid { data }
}
