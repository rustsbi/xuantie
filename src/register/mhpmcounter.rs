//! mhpmcounter, Xuantie performance counter

/// mhpmcounter3: L1 I-cache access counter
pub fn l1_i_cache_access() -> usize {
    unsafe { get_csr_value!(0xB03) }
}
/// mhpmcounter4: L1 I-cache miss counter
pub fn l1_i_cache_miss() -> usize {
    unsafe { get_csr_value!(0xB04) }
}
/// mhpmcounter5: I-uTLB miss counter
pub fn i_utlb_miss() -> usize {
    unsafe { get_csr_value!(0xB05) }
}
/// mhpmcounter6: D-uTLB miss counter
pub fn d_utlb_miss() -> usize {
    unsafe { get_csr_value!(0xB06) }
}
/// mhpmcounter7: jTLB miss counter
pub fn jtlb_miss() -> usize {
    unsafe { get_csr_value!(0xB07) }
}
/// mhpmcounter8: Conditional branch mispredict counter
pub fn conditional_branch_mispredict() -> usize {
    unsafe { get_csr_value!(0xB08) }
}
/// mhpmcounter9: Conditional branch instruction counter
pub fn conditional_branch_instruction() -> usize {
    unsafe { get_csr_value!(0xB09) }
}
/// mhpmcounter13: Store instruction counter
pub fn store_instruction() -> usize {
    unsafe { get_csr_value!(0xB0D) }
}
/// mhpmcounter14: L1 D-cache read access counter
pub fn l1_d_cache_read_access() -> usize {
    unsafe { get_csr_value!(0xB0E) }
}
/// mhpmcounter15: L1 D-cache read miss counter
pub fn l1_d_cache_read_miss() -> usize {
    unsafe { get_csr_value!(0xB0F) }
}
/// mhpmcounter16: L1 D-cache write access counter
pub fn l1_d_cache_write_access() -> usize {
    unsafe { get_csr_value!(0xB10) }
}
/// mhpmcounter17: L1 D-cache write miss counter
pub fn l1_d_cache_write_miss() -> usize {
    unsafe { get_csr_value!(0xB11) }
}
// 10..=12, 18..=31: undefined
