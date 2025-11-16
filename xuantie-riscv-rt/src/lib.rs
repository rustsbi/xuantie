//! Startup code and minimal runtime for T-Head Xuantie RISC-V CPUs
#![no_std]

pub mod arch;
pub mod interrupts;
pub mod stack;

extern "C" {
    fn main();
}
