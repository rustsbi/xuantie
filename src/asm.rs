//! Assembly instructions
//!
//! This module allows you to use XuanTie instructions without using specialized assembler or compiler.
//!
//! Not all these instructions are supported on your XuanTie platform. 
//! You may use `mcpuid` register to get your implementation model, or read the manual
//! before using any of following assembly instructions.

/// DCACHE.CALL, D-cache clean all dirty items instruction
///
/// Clears all L1 D-cache table elements, write all dirty items to next level storage.
///
/// # Permissions
///
/// Must run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
#[inline]
pub unsafe fn dcache_call() {
    asm!(".word 0x0010000B")
}

/// DCACHE.CVA, D-cache clean dirty item for virtual address instruction
///
/// Writes D-cache table item corresponding to virtual address `va` to next level storage.
/// This operation effects on L1 cache on all cores.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
/// 
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 1`, this instruction can be run on U mode.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 0`, 
///   this instruction will raise illegal instruction when being run U mode.
#[inline]
pub unsafe fn dcache_cva(va: usize) {
    asm!(".word 0x0245000B", in("a0") va)
}

/// DCACHE.CPA, D-cache clean dirty item for physical address instruction
#[inline]
pub unsafe fn dcache_cpa(pa: usize) {
    asm!(".word 0x0285000B", in("a0") pa)
}

/// DCACHE.CSW, D-cache clean dirty item for way or set instruction
#[inline]
pub unsafe fn dcache_csw(set_or_way: usize) {
    asm!(".word 0x0215000B", in("a0") set_or_way)
}

/// DCACHE.IALL, D-cache invalid all items instruction
#[inline]
pub unsafe fn dcache_iall() {
    asm!(".word 0x0200000B")
}

/// DCACHE.IVA, D-cache invalid item for virtual address instruction
#[inline]
pub unsafe fn dcache_iva(va: usize) {
    asm!(".word 0x0265000B", in("a0") va)
}

#[inline]
/// DCACHE.IPA, D-cache invalid item for physical address instruction
pub unsafe fn dcache_ipa(pa: usize) {
    asm!(".word 0x02A5000B", in("a0") pa)
}

/// DCACHE.ISW, D-cache invalid item for way or set instruction
#[inline]
pub unsafe fn dcache_isw(set_or_way: usize) {
    asm!(".word 0x0225000B", in("a0") set_or_way)
}

/// DCACHE.CIALL, D-cache clean all dirty and invalid item instruction
#[inline]
pub unsafe fn dcache_ciall() {
    asm!(".word 0x0300000B")
}

/// DCACHE.CIVA, D-cache clean dirty and invalid for virtual address instruction
#[inline]
pub unsafe fn dcache_civa(va: usize) {
    asm!(".word 0x0275000B", in("a0") va)
}

/// DCACHE.CIPA, D-cache clean dirty and invalid for physical address instruction
#[inline]
pub unsafe fn dcache_cipa(pa: usize) {
    asm!(".word 0x02B5000B", in("a0") pa)
}

/// DCACHE.CISW, D-cache clean dirty and invalid for set or way instruction
#[inline]
pub unsafe fn dcache_cisw(set_or_way: usize) {
    asm!(".word 0x0235000B", in("a0") set_or_way)
}

/// ICACHE.IALL, I-cache invalid all items instruction
#[inline]
pub unsafe fn icache_iall() {
    asm!(".word 0x0100000B")
}

/// ICACHE.IALLS, I-cache broadcast all cores to invalid all items instruction
#[inline]
pub unsafe fn icache_ialls() {
    asm!(".word 0x0110000B")
}

/// ICACHE.IVA, I-cache invalid item for virtual address instruction
#[inline]
pub unsafe fn icache_iva(va: usize) {
    asm!(".word 0x0305000B", in("a0") va)
}

/// ICACHE.IPA, I-cache invalid item for physical address instruction
#[inline]
pub unsafe fn icache_ipa(pa: usize) {
    asm!(".word 0x0385000B", in("a0") pa)
}

/// IPUSH, fast interrupt stack push instruction
///
/// Push interrupt switch registers into current stack. 
/// It pushes `mcause`, `mepc`, `x1`, `x5` to `x7`, `x10` to `x17` and `x28` to `x31` into stack.
/// Another word, the pushed `xi` integer registers are `ra`, `t0` to `t6`, and `a0` to `a7` (not in order)
/// other than CSR registers `mcause` and `mepc`.
///
/// In pseudocode, it performs like:
///
/// ```no_run
/// *sp.offset(-1) = mcause;
/// *sp.offset(-2) = mepc;
/// *sp.offset(-3) = ra;
/// /* ... Mem[sp - 4] ..= Mem[sp - 72] ← mcause, mepc, {xi} */
/// *sp.offset(-18) = t6;
/// sp = sp.offset(-18);
/// ```
///
/// # Permissions
///
/// Must run on M mode.
///
/// # Exceptions
///
/// Raises store unaligned exception, store access exception, or illegal instruction exception.
#[inline]
pub unsafe fn ipush() {
    asm!(".word 0x0040000B")
}

/// IPOP, fast interrupt stack pop instruction
///
/// Pop interrupt switch registers from current stack, and return from interrupt environment. 
/// It pops `mcause`, `mepc`, `x1`, `x5` to `x7`, `x10` to `x17` and `x28` to `x31` from stack.
/// Another word, the poped `xi` integer registers are `ra`, `t0` to `t6`, and `a0` to `a7` (not in order)
/// other than CSR registers `mcause` and `mepc`.
///
/// In pseudocode, it performs like:
///
/// ```no_run
/// mcause = *sp.offset(17);
/// mepc = *sp.offset(16);
/// ra = *sp.offset(15);
/// /* ... mcause, mepc, {xi} ← Mem[sp + 68] ..= Mem[sp] */
/// t6 = *sp.offset(0);
/// sp = sp.offset(18);
/// riscv::asm::mret();
/// ```
///
/// # Permissions
///
/// Must run on M mode.
///
/// # Exceptions
///
/// Raises store unaligned exception, store access exception, or illegal instruction exception.
#[inline]
pub unsafe fn ipop() {
    asm!(".word 0x0050000B")
}
