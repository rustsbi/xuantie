//! mcindex, machine cache visit index register
use bit_field::BitField;
use core::arch::asm;

/// RAM information
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RID {
    /// Instruction cache tag RAM
    ICacheTag = 0,
    /// Instruction cache data RAM
    ICacheData = 1,
    /// Data cache tag RAM
    DCacheTag = 2,
    /// Data cache data RAM
    DCacheData = 3,
}

/// Set RAM to visit
#[inline]
pub unsafe fn set_rid(rid: RID) {
    let mut value: usize;
    asm!("csrr {}, 0x7D3", out(reg) value);
    value.set_bits(28..=29, rid as usize);
    asm!("csrw 0x7D3, {}", in(reg) value);
}

/// Set way in level 1
#[inline]
pub unsafe fn set_way_l1(way: u8) {
    assert!(way <= 0b1111);
    let mut value: usize;
    asm!("csrr {}, 0x7D3", out(reg) value);
    value.set_bits(17..=20, way as usize);
    asm!("csrw 0x7D3, {}", in(reg) value);
}

/// Set way in level 2
#[inline]
pub unsafe fn set_way_l2(way: u8) {
    assert!(way <= 0b1111);
    let mut value: usize;
    asm!("csrr {}, 0x7D3", out(reg) value);
    value.set_bits(21..=24, way as usize);
    asm!("csrw 0x7D3, {}", in(reg) value);
}

/// Set cache index
///
/// Lower 4 for 6 bits may be ignored according to RAM type and actual SoC implementation.
#[inline]
pub unsafe fn set_index(index: u32) {
    assert!(index <= 0x1FFFF);
    let mut value: usize;
    asm!("csrr {}, 0x7D3", out(reg) value);
    value.set_bits(0..=16, index as usize);
    asm!("csrw 0x7D3, {}", in(reg) value);
}
