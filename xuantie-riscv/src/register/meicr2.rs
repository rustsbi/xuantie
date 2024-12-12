//! meicr2, L2-cache hardware fault inject register
use core::arch::asm;

/// L2 error controllable RAM index
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum L2RAMID {
    /// L2-cache tag ram
    L2CacheTag = 0,
    /// L2-cache data ram
    L2CacheData = 1,
    /// L2-cache dirty ram
    L2CacheDirty = 2,
}

set_clear_csr! {
    /// L2-cache error control error inject enable
    , 0x7D7, set_inj_en, clear_inj_en, 1 << 0
}
set_clear_csr! {
    /// Error control error fatal inject enable
    , 0x7D7, set_fatal_inj, clear_fatal_inj, 1 << 1
}

/// Inject hardware fault
///
/// If `fatal_inj` is `1`, inject a 2-bit error; if `fatal_inj` is `0`, inject a 1-bit error.
/// Set `inj_en` to `1` to start L2-cache error control error injection.
#[inline]
pub unsafe fn write(inj_en: bool, fatal_inj: bool, ramid: L2RAMID) {
    let bits = inj_en as usize | ((fatal_inj as usize) << 1) | ((ramid as usize) << 29);
    asm!("csrw 0x7D7, {}", in(reg) bits)
}
