//! mrvbr, machine reset vector base address register

read_csr!(0x7C7);

/// Get reset vector base address
#[inline]
pub fn get_rvbr() -> usize {
    unsafe { _read() }
}
