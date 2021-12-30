//! meicr, L1-cache hardware fault inject register
use core::arch::asm;

pub use super::mcer::RAMID;

set_clear_csr! {
    /// L1-cache error control error inject enable
    , 0x7D6, set_inj_en, clear_inj_en, 1 << 0
}
set_clear_csr! {
    /// Error control error fatal inject enable
    , 0x7D6, set_fatal_inj, clear_fatal_inj, 1 << 1
}

/// Inject hardware fault
///
/// If `fatal_inj` is `1`, inject a 2-bit error; if `fatal_inj` is `0`, inject a 1-bit error.
/// Set `inj_en` to `1` to start L1-cache error control error injection.
#[inline]
pub unsafe fn write(inj_en: bool, fatal_inj: bool, ramid: RAMID) {
    let bits = inj_en as usize | ((fatal_inj as usize) << 1) | ((ramid as usize) << 29);
    asm!("csrw 0x7D6, {}", in(reg) bits)
}
