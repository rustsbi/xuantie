//! mcdata{0,1}, machine cache data registers
use bit_field::BitField;
use core::arch::asm;

/// I-cache tag RAM visit result
#[derive(Clone, Copy, Debug)]
pub struct ICacheTag {
    pub tag: u32,
    pub valid: bool,
}

/// Get I-cache tag visit result
#[inline]
pub fn get_icache_tag() -> ICacheTag {
    let mcdata0 = read_mcdata0();
    ICacheTag {
        tag: mcdata0.get_bits(12..=39) as u32,
        valid: mcdata0.get_bit(0),
    }
}

/// I-cache data RAM visit result
#[derive(Clone, Copy, Debug)]
pub struct ICacheData {
    pub data: u128,
}

/// Get I-cache data visit result
#[inline]
pub fn get_icache_data() -> ICacheData {
    let (mcdata0, mcdata1) = read_mcdata();
    ICacheData {
        data: (mcdata0 as u128) | ((mcdata1 as u128) << 64),
    }
}

/// D-cache tag RAM visit result
#[derive(Clone, Copy, Debug)]
pub struct DCacheTag {
    pub tag: u32,
    pub dirty: bool,
    pub valid: bool,
}

/// Get D-cache tag visit result
#[inline]
pub fn get_dcache_tag() -> DCacheTag {
    let mcdata0 = read_mcdata0();
    DCacheTag {
        tag: mcdata0.get_bits(12..=39) as u32,
        dirty: mcdata0.get_bit(2),
        valid: mcdata0.get_bit(0),
    }
}

/// D-cache data RAM visit result
#[derive(Clone, Copy, Debug)]
pub struct DCacheData {
    pub data: u128,
}

/// Get D-cache data visit result
#[inline]
pub fn get_dcache_data() -> DCacheData {
    let (mcdata0, mcdata1) = read_mcdata();
    DCacheData {
        data: (mcdata0 as u128) | ((mcdata1 as u128) << 64),
    }
}

#[inline]
fn read_mcdata() -> (usize, usize) {
    let (mcdata0, mcdata1);
    unsafe {
        asm!(
            "csrr {0}, 0x7D4",
            "csrr {1}, 0x7D5",
            out(reg) mcdata0,
            out(reg) mcdata1,
        );
    }
    (mcdata0, mcdata1)
}

#[inline]
fn read_mcdata0() -> usize {
    let mcdata0;
    unsafe {
        asm!("csrr {}, 0x7D4", out(reg) mcdata0);
    }
    mcdata0
}
