//! mraddr, machine reset address register

read_csr!(0x7E0);

/// Get machine reset address
#[inline]
pub fn read() -> usize {
    unsafe { _read() }
}
