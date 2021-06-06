//! mnmipc, machine NMI exception program counter

read_csr!(0x7E3);

/// Get NMI exception program counter
#[inline]
pub fn read() -> usize {
    unsafe { _read() }
}
