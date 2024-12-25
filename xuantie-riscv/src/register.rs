//! XuanTie extended CSRs.

// Extended state registers for performance cores
pub mod mxstatus; // 0x7C0 
pub mod mhcr; // 0x7C1
pub mod mcor; // 0x7C2
pub mod mccr2; // 0x7C3, mccr2h 0x7CB RV32
pub mod mcer2; // 0x7C4, mcer2h 0x7FF RV32
pub mod mhint; // 0x7C5, mhint2h 0x7CA RV32
pub mod mrmr; // 0x7C6
pub mod mrvbr; // 0x7C7
pub mod mcer; // 0x7C8
// pub mod mcounterwen; // 0x7C9
// pub mod mcounterinten; // 0x7CA
// pub mod mcounterof; // 0x7CB

// Extended supervisor state registers
pub mod sxstatus; // 0x5C0 
pub mod shcr; // 0x5C1
pub mod scer2; // 0x5C2, scer2h 0x5D4 RV32
pub mod scer; // 0x5C3, scerh 0x5D3 RV32
// pub mod scounterinten; // 0x5C4
// pub mod scounterof; // 0x5C5

// Extended supervisor virtual memory registers
// pub mod smir; // 0x9C0
pub mod smel; // 0x9C1
pub mod smeh; // 0x9C2
// pub mod smcir; // 0x9C3

// Extended state registers for embedded cores
pub mod mraddr; // 0x7E0
pub mod mexstatus; // 0x7E1
pub mod mnmicause; // 0x7E2
pub mod mnmipc; // 0x7E3

// Extended float point registers
pub mod fxcr; // 0x800

// Performance counter
pub mod mhpmcounter;
// pub mod shpmcounter; // 0x5E3..=0x5FF

// Cache direct access registers
pub mod mcins; // 0x7D2
pub mod mcindex; // 0x7D3, mcindexh 0x7DA RV32
pub mod mcdata; // mcdata0 0x7D4, mcdata1 0x7D5, mcdata0h 0x7DB RV32, mcdata1h 0x7DB? RV32
pub mod meicr; // 0x7D6
pub mod meicr2; // 0x7D7
// pub mod mbeaddr; // 0x7D8

// Processor identification registers
pub mod mcpuid; // 0xFC0
pub mod mapbaddr; // 0xFC1
// pub mod mapbaddr2; // 0xFC3
