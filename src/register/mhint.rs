//! mhint, machine hint register
use bit_field::BitField;
use core::arch::asm;

/// L1 D-cache write allocation strategy
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AMR {
    /// Depends on WA page entry attibute
    WA = 0,
    /// Don't write to L1 cache after 3 continuous cache line writes
    After3Lines = 1,
    /// Don't write to L1 cache after 64 continuous cache line writes
    After64Lines = 2,
    /// Don't write to L1 cache after 128 continuous cache line writes
    After128Lines = 3,
}

/// D-cache prefetch lines
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PrefN {
    /// Prefetch 2 lines
    TwoLines = 0,
    /// Prefetch 4 lines
    FourLines = 1,
    /// Prefetch 8 lines
    EightLines = 2,
    /// Prefetch 16 lines
    SixteenLines = 3,
}

set_clear_csr! {
    /// D-cache prefetch enable
    , 0x7C5, set_dpld, clear_dpld, 1 << 2
}
set_clear_csr! {
    /// I-cache prefetch enable
    , 0x7C5, set_ipld, clear_ipld, 1 << 8
}

/// Set D-cache write allocation strategy
#[inline]
pub unsafe fn set_amr(amr: AMR) {
    let mut value: usize;
    asm!("csrr {}, 0x7C5", out(reg) value);
    value.set_bits(3..=4, amr as usize);
    asm!("csrw 0x7C5, {}", in(reg) value);
}

/// Set D-cache prefetch lines configuration
#[inline]
pub unsafe fn set_prefn(prefn: PrefN) {
    let mut value: usize;
    asm!("csrr {}, 0x7C5", out(reg) value);
    value.set_bits(13..=14, prefn as usize);
    asm!("csrw 0x7C5, {}", in(reg) value);
}
