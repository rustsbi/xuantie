//! Assembly instructions.
//!
//! This module allows you to use XuanTie instructions without using specialized assembler or compiler.
//!
//! Not all these instructions are supported on your XuanTie platform.
//! You may use `mcpuid` register to get your implementation model, or read the manual
//! before using any of following assembly instructions.

mod xtheadcmo;
pub use xtheadcmo::*;
mod xtheadsync;
pub use xtheadsync::*;
mod xtheadint;
pub use xtheadint::*;

mod dsp0p9;
pub use dsp0p9::*;
