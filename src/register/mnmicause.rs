//! mnmicause, machine NMI state register
use bit_field::BitField;
use core::arch::asm;

/// mnmicause register
#[derive(Clone, Copy, Debug)]
pub struct Mnmicause {
    bits: usize,
}

/// Machine Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MPP {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

impl Mnmicause {
    /// NMI vector exception code in mcause
    #[inline]
    pub fn nmi_vector(&self) -> usize {
        self.bits.get_bits(0..=11)
    }
    /// NMI mstatus previous interrupt enable
    #[inline]
    pub fn nmi_mpie(&self) -> bool {
        self.bits.get_bit(27)
    }
    /// NMI mstatus previous privilege mode
    #[inline]
    pub fn nmi_mpp(&self) -> MPP {
        match self.bits.get_bits(28..=29) {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }
    /// NMI INTR (is interrupt) value bit in mcause
    #[inline]
    pub fn nmi_intr(&self) -> bool {
        self.bits.get_bit(31)
    }
}

read_csr_as!(Mnmicause, 0x7E2);
