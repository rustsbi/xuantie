//! mcindex, machine cache visit index register
//!
//! The current C910/C920 manual describes `RID` as bits 31:28. It also lists
//! I-cache predecode RAM as RID 19, which cannot be represented by that four-bit
//! field. This module exposes all RID values from 0 through 13 that are
//! representable by the documented layout.
use bit_field::BitField;
use core::arch::asm;

/// RAM information
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Rid {
    /// Instruction cache tag RAM
    ICacheTag = 0,
    /// Instruction cache data RAM
    ICacheData = 1,
    /// Data cache store-tag RAM
    DCacheTag = 2,
    /// Data cache data RAM
    DCacheData = 3,
    /// L2-cache tag RAM
    L2CacheTag = 4,
    /// L2-cache data RAM
    L2CacheData = 5,
    /// Instruction cache tag ECC RAM
    ICacheTagEcc = 6,
    /// Instruction cache data ECC RAM
    ICacheDataEcc = 7,
    /// Data cache store-tag ECC RAM
    DCacheStTagEcc = 8,
    /// Data cache data ECC RAM
    DCacheDataEcc = 9,
    /// L2-cache tag ECC RAM
    L2CacheTagEcc = 10,
    /// L2-cache data ECC RAM
    L2CacheDataEcc = 11,
    /// Data cache load-tag RAM
    DCacheLdTag = 12,
    /// Data cache load-tag ECC RAM
    DCacheLdTagEcc = 13,
}

/// Set RAM to visit.
#[inline]
pub unsafe fn set_rid(rid: Rid) {
    let mut value: usize;
    asm!("csrr {}, 0x7D3", out(reg) value);
    value = with_rid(value, rid);
    asm!("csrw 0x7D3, {}", in(reg) value);
}

/// Set the cache way.
#[inline]
pub unsafe fn set_way(way: u8) {
    let mut value: usize;
    asm!("csrr {}, 0x7D3", out(reg) value);
    value = with_way(value, way);
    asm!("csrw 0x7D3, {}", in(reg) value);
}

/// Set cache index.
///
/// Lower 4 or 6 bits may be ignored according to RAM type and actual SoC
/// implementation.
#[inline]
pub unsafe fn set_index(index: u32) {
    let mut value: usize;
    asm!("csrr {}, 0x7D3", out(reg) value);
    value = with_index(value, index);
    asm!("csrw 0x7D3, {}", in(reg) value);
}

#[inline]
fn with_rid(mut value: usize, rid: Rid) -> usize {
    value.set_bits(28..=31, rid as usize);
    value
}

#[inline]
fn with_way(mut value: usize, way: u8) -> usize {
    assert!(way <= 0xF);
    value.set_bits(21..=24, way as usize);
    value
}

#[inline]
fn with_index(mut value: usize, index: u32) -> usize {
    assert!(index <= 0x1F_FFFF);
    value.set_bits(0..=20, index as usize);
    value
}

#[cfg(test)]
mod tests {
    use super::{with_index, with_rid, with_way, Rid};

    #[test]
    fn writes_documented_fields_without_overlap() {
        let original = usize::MAX;

        let value = with_index(original, 0x12_3456);
        assert_eq!(value & 0x1F_FFFF, 0x12_3456);
        assert_eq!(value & !0x1F_FFFF, original & !0x1F_FFFF);

        let value = with_way(original, 0xA);
        assert_eq!((value >> 21) & 0xF, 0xA);
        assert_eq!(value & !(0xF << 21), original & !(0xF << 21));

        let value = with_rid(original, Rid::DCacheLdTagEcc);
        assert_eq!((value >> 28) & 0xF, 13);
        assert_eq!(value & !(0xF << 28), original & !(0xF << 28));
    }

    #[test]
    fn all_representable_rids_fit_the_four_bit_field() {
        let rids = [
            Rid::ICacheTag,
            Rid::ICacheData,
            Rid::DCacheTag,
            Rid::DCacheData,
            Rid::L2CacheTag,
            Rid::L2CacheData,
            Rid::ICacheTagEcc,
            Rid::ICacheDataEcc,
            Rid::DCacheStTagEcc,
            Rid::DCacheDataEcc,
            Rid::L2CacheTagEcc,
            Rid::L2CacheDataEcc,
            Rid::DCacheLdTag,
            Rid::DCacheLdTagEcc,
        ];

        assert!(rids.into_iter().all(|rid| (rid as u8) <= 0xF));
    }
}
