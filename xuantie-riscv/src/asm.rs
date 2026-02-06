//! Assembly instructions.
//!
//! This module allows you to use XuanTie instructions without using specialized assembler or compiler.
//!
//! Not all these instructions are supported on your XuanTie platform.
//! You may use the XuanTie [`mcpuid`] register to get your implementation model, or read the manual
//! before using any of following assembly instructions.
//!
//! [`mcpuid`]: ../register/mcpuid/index.html

mod xtheadcmo;
pub use xtheadcmo::*;
mod xtheadsync;
pub use xtheadsync::*;
mod xtheadint;
pub use xtheadint::*;

// Available on some T-Head embedded cores, but not listed in XThead specification.
pub mod dsp0p9;
