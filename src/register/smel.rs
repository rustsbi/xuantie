//! smel, supervisor memory entry low register

use crate::paging::Entry;

read_csr!(0x9C1);

/// Reads the smel register
#[inline]
pub fn read() -> Entry {
    let bits = unsafe { _read() };
    Entry::from_bits(bits)
}
