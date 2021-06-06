//! mhpmcounter, Xuantie performance counter

/// mhpmcounter3: L1 I-cache access counter
pub fn l1_i_cache_access() -> usize {
    unsafe { get_csr_value!(0x323) }
}
/// mhpmcounter4: L1 I-cache miss counter
pub fn l1_i_cache_miss() -> usize {
    unsafe { get_csr_value!(0x324) }
}
/// mhpmcounter5: I-uTLB miss counter
pub fn i_utlb_miss() -> usize {
    unsafe { get_csr_value!(0x325) }
}
/// mhpmcounter6: D-uTLB miss counter
pub fn d_utlb_miss() -> usize {
    unsafe { get_csr_value!(0x326) }
}
/// mhpmcounter7: jTLB miss counter
pub fn jtlb_miss() -> usize {
    unsafe { get_csr_value!(0x327) }
}
/// mhpmcounter8: Conditional branch mispredict counter
pub fn conditional_branch_mispredict() -> usize {
    unsafe { get_csr_value!(0x328) }
}
/// mhpmcounter9: Conditional branch instruction counter
pub fn conditional_branch_instruction() -> usize {
    unsafe { get_csr_value!(0x329) }
}
/// mhpmcounter13: Store instruction counter
pub fn store_instruction() -> usize {
    unsafe { get_csr_value!(0x32D) }
}
/// mhpmcounter14: L1 D-cache read access counter
pub fn l1_d_cache_read_access() -> usize {
    unsafe { get_csr_value!(0x32E) }
}
/// mhpmcounter15: L1 D-cache read miss counter
pub fn l1_d_cache_read_miss() -> usize {
    unsafe { get_csr_value!(0x32F) }
}
/// mhpmcounter16: L1 D-cache write access counter
pub fn l1_d_cache_write_access() -> usize {
    unsafe { get_csr_value!(0x330) }
}
/// mhpmcounter17: L1 D-cache write miss counter
pub fn l1_d_cache_write_miss() -> usize {
    unsafe { get_csr_value!(0x331) }
}
// 10..=12, 18..=31: undefined
