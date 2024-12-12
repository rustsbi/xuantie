//! mcins, machine cache instruction register
use core::arch::asm;

set_clear_csr! {
    /// Cache read request
    , 0x7D2, set_r, clear_r, 1 << 0
}
