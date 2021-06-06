//! mcindex, machine cache visit index register

set!(0x7D3);
clear!(0x7D3);

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
    _set((rid as usize) << 28)
}

/// Set way in level 1
#[inline]
pub unsafe fn set_way_l1(way: u8) {
    assert!(way <= 0b1111);
    _set((way as usize) << 17)
}

/// Set way in level 2
#[inline]
pub unsafe fn set_way_l2(way: u8) {
    assert!(way <= 0b1111);
    _set((way as usize) << 21)
}

/// Set cache index
///
/// Lower 4 for 6 bits may be ignored according to RAM type and actual SoC implementation.
#[inline]
pub unsafe fn set_index(index: u16) {
    _set((index as usize) << 0)
}
