#![feature(asm)]
#![no_std]

#[macro_use]
mod macros;
pub mod asm;
pub mod register;
pub mod paging;
pub mod debug;
