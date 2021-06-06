//! mapbaddr, machine APB base address register

read_csr!(0xFC1);

/// Get APB peripheral base address
#[inline]
pub fn read() -> usize {
    unsafe { _read() }
}
