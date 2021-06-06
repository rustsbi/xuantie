//! Xuantie extended CSRs

// Extended status registers
pub mod mxstatus;
pub mod mhcr;
pub mod mcor;
pub mod mhint;
pub mod mrvbr;
// pub mod mcer;
// pub mod mcounterwen;
// pub mod mcounterinten;
// pub mod mcounterod;
pub mod mapbaddr;

// Performance counter
pub mod mhpmevent;

// Cache direct access registers
pub mod mcins;
pub mod mcindex;
pub mod mcdata;

// Processor identification registers
pub mod mcpuid;
