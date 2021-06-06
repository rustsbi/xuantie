//! mrmr, machine reset signal register

set!(0x7C6);
clear!(0x7C6);
read_csr!(0x7C6);

/// Write to mrmr register to release reset lock for given harts
///
/// Write `0x1` for hart 0, `0x2` for 1, `0x4` for 2, etc.
#[inline]
pub unsafe fn write(bits: usize) {
    _set(bits)
}
