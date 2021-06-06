//! meicr, L1-cache hardware fault inject register

set!(0x7D6);
clear!(0x7D6);
write_csr!(0x7D6);

/// ECC RAM index
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RAMID {
    /// Instruction cache tag ram
    ICacheTag = 0,
    /// Instruction cache data ram
    ICacheData = 1,
    /// Data cache tag ram
    DCacheTag = 2,
    /// Data cache data ram
    DCacheData = 3,
    /// jTLB tag ram
    JTlbTag = 4,
    /// jTLB data ram
    JTlbData = 5,
}

set_clear_csr! {
    /// L1-cache ECC error inject enable
    , set_inj_en, clear_inj_en, 1 << 0
}
set_clear_csr! {
    /// ECC error fatal inject enable
    , set_fatal_inj, clear_fatal_inj, 1 << 1
}

/// Inject hardware fault
///
/// If `fatal_inj` is `1`, inject a 2-bit error; if `fatal_inj` is `0`, inject a 1-bit error.
/// Set `inj_en` to `1` to start L1-cache ECC error injection.
#[inline]
pub unsafe fn write(inj_en: bool, fatal_inj: bool, ramid: RAMID) {
    let bits = inj_en as usize | ((fatal_inj as usize) << 1) | ((ramid as usize) << 29);
    _write(bits)
}
