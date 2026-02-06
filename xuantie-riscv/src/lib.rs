//! Low level access to T-Head XuanTie RISC-V processors.
//!
//! This crate follows the T-Head ISA extension specification (Xthead*), version 2.3.0, 2023-11-10.
//! The specification is open sourced and can be downloaded [here](https://github.com/XUANTIE-RV/thead-extension-spec).
//!
//! # Examples
//!
//! Here's a list of example code of how to using this library under different scenarios.
//!
//! ## Enable and invalidate caches
//!
//! C906 will invalid both I-cache and D-cache automatically when reset.
//! Use the following operation, we can enable I-cache and D-cache.
//!
//! ```no_run
//! use xuantie_riscv::register::mhcr;
//! // enable D-cache
//! unsafe { mhcr::set_de() };
//! // enable I-cache
//! unsafe { mhcr::set_ie() };
//! ```
//!
//! Additionally, we can invalidate I-cache or D-cache manually if necessary.
//!
//! ```no_run
//! // If we don't have `mxstatus.theadisaee` enabled
//! use xuantie_riscv::register::mcor::{self, Cache, Operation};
//! // invalidate caches. Use `Cache::DATA` or `Cache::INSTRUCION`
//! // to invalid a specific cache.
//! unsafe { mcor::cache(Cache::BOTH, Operation::INVALIDATE); }
//! ```
//! ```no_run
//! // If we have `mxstatus.theadisaee` enabled
//! use xuantie_riscv::asm::{icache_iall, dcache_iall, sync_is};
//! // invalidate I-cache
//! unsafe { icache_iall(); sync_is(); }
//! // invalidate D-cache
//! unsafe { dcache_iall(); sync_is(); }
//! ```
//!
//! ## CPU power down
//!
//! The following code would power down a Xuantie C906 core.
//! This procedure is useful when developing SBI implementations.
//!
//! Make sure the hart is at M mode when executing the following code.
//!
//! ```no_run
//! # mod riscv { pub mod register { pub mod mstatus { pub fn clear_mie() {} }} pub mod asm { pub fn wfi() {}}}
//! use riscv::{asm::wfi, register::mstatus};
//! use xuantie_riscv::{asm::dcache_call, register::{mhint, mhcr}};
//! // power down the current processor hart
//! unsafe fn shutdown() -> ! {
//!     // disable interrupt
//!     mstatus::clear_mie();
//!     // disable data prefetch
//!     mhint::clear_dpld();
//!     // clear D-cache
//!     dcache_call();
//!     // disable D-cache
//!     mhcr::clear_de();
//!     // execute wait-for-interrupt instruction
//!     wfi();
//!     // no instruction would be able to wake this WFI
//!     unreachable!()
//! }
//! ```
#![no_std]

#[macro_use]
mod macros;
pub mod asm;
pub mod debug;
pub mod paging;
pub mod peripheral;

#[rustfmt::skip]
pub mod register;
