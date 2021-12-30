//! mhint, machine hint register

set!(0x7C5);
clear!(0x7C5);

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
pub enum DDIS {
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
    , set_dpld, clear_dpld, 1 << 2
}
set_clear_csr! {
    /// I-cache prefetch enable
    , set_ipld, clear_ipld, 1 << 8
}

/// Set D-cache write allocation strategy
#[inline]
pub unsafe fn set_amr(value: AMR) {
    _set((value as usize) << 3)
}

/// Set D-cache prefetch lines configuration
#[inline]
pub unsafe fn set_ddis(value: DDIS) {
    _set((value as usize) << 13)
}
