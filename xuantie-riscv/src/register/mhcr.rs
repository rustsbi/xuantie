//! mhcr, machine hardware configuration register
//!
//! # Platform support
//!
//! This register is supported on Xuantie C920, C910, C906, E907, E906 and E902 cores.
//!
//! # E907 and E906 layouts
//!
//! Xuantie E907 uses a different layout for several fields: `WB` is bit 2, `WA` is
//! bit 3, and `BTB` is bit 12. Use the accessors and modifiers with the `_e907`
//! suffix for those fields. E906 uses the same positions for `WB` and `WA`, and
//! identifies bit 12 as `L0BTB`.
use bit_field::BitField;

/// mhcr register
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Mhcr {
    bits: usize,
}

impl Mhcr {
    /// I-cache enable
    ///
    /// # Platform support
    ///
    /// This bit is supported on Xuantie C920, C910, C906, E907, E906 and E902 cores.
    #[inline]
    pub fn ie(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// D-cache enable
    ///
    /// # Platform support
    ///
    /// This bit is supported on Xuantie C920, C910, C906, E907 and E906 cores.
    #[inline]
    pub fn de(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Cache write allocate configuration enable
    ///
    /// # Platform support
    ///
    /// This accessor supports Xuantie C920, C910 and C906 cores.
    #[inline]
    pub fn wa(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Write back status
    ///
    /// # Platform support
    ///
    /// This bit is fixed at one on Xuantie C920, C910 and C906 cores.
    #[inline]
    pub fn wb(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// Cache write allocate configuration enable on Xuantie E907 and E906.
    #[inline]
    pub fn wa_e907(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// Write back enable on Xuantie E907 and E906.
    #[inline]
    pub fn wb_e907(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Return stack enable
    ///
    /// # Platform support
    ///
    /// This bit is supported on Xuantie C920, C910, C906, E907 and E906 cores.
    #[inline]
    pub fn rs(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Branch predict enable
    ///
    /// # Platform support
    ///
    /// This bit is supported on Xuantie C920, C910, C906, E907 and E906 cores.
    #[inline]
    pub fn bpe(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Branch target buffer enable
    ///
    /// # Platform support
    ///
    /// This accessor supports Xuantie C920, C910 and C906 cores.
    #[inline]
    pub fn btb(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Branch target buffer enable on Xuantie E907.
    #[inline]
    pub fn btb_e907(&self) -> bool {
        self.bits.get_bit(12)
    }

    /// Write bulk transfer enable
    ///
    /// # Platform support
    ///
    /// This bit is fixed at one on Xuantie C920, C910 and C906 cores.
    #[inline]
    pub fn wbr(&self) -> bool {
        self.bits.get_bit(8)
    }
}

read_csr_as!(Mhcr, 0x7C1);

set_clear_csr! {
    /// I-cache enable
    , 0x7C1, set_ie, clear_ie, 1 << 0
}
set_clear_csr! {
    /// D-cache enable
    , 0x7C1, set_de, clear_de, 1 << 1
}
set_clear_csr! {
    /// Cache write allocate configuration enable
    ///
    /// This modifier does not apply to Xuantie E907 or E906.
    , 0x7C1, set_wa, clear_wa, 1 << 2
}
set_clear_csr! {
    /// Return stack enable
    , 0x7C1, set_rs, clear_rs, 1 << 4
}
set_clear_csr! {
    /// Branch predict enable
    , 0x7C1, set_bpe, clear_bpe, 1 << 5
}
set_clear_csr! {
    /// Branch target buffer enable
    ///
    /// This modifier does not apply to Xuantie E907 or E906.
    , 0x7C1, set_btb, clear_btb, 1 << 6
}
set_clear_csr! {
    /// Cache write allocate configuration enable on Xuantie E907 and E906
    , 0x7C1, set_wa_e907, clear_wa_e907, 1 << 3
}
set_clear_csr! {
    /// Write back enable on Xuantie E907 and E906
    , 0x7C1, set_wb_e907, clear_wb_e907, 1 << 2
}
set_clear_csr! {
    /// Branch target buffer enable on Xuantie E907
    , 0x7C1, set_btb_e907, clear_btb_e907, 1 << 12
}

#[cfg(test)]
mod tests {
    use super::Mhcr;

    #[test]
    fn decodes_c_series_and_e907_layouts_separately() {
        let mhcr = Mhcr {
            bits: (1 << 2) | (1 << 12),
        };

        assert!(mhcr.wa());
        assert!(!mhcr.wb());
        assert!(!mhcr.btb());

        assert!(!mhcr.wa_e907());
        assert!(mhcr.wb_e907());
        assert!(mhcr.btb_e907());
    }
}
