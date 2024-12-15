use core::arch::asm;

// T-Head extended instructions are mostly encoded under custom-0 opcode space (opcode 0x0B).

/// Fast interrupt stack push instruction.
///
/// Push interrupt switch registers into current stack.
/// It pushes `mcause`, `mepc`, `x1`, `x5` to `x7`, `x10` to `x17` and `x28` to `x31` into stack.
/// Another word, the pushed `xi` integer registers are `ra`, `t0` to `t6`, and `a0` to `a7` (not in order)
/// other than CSR registers `mcause` and `mepc`.
///
/// In pseudocode, it performs like:
///
/// ```no_run
/// # let mut sp: *mut u8 = core::ptr::null_mut();
/// # let (mcause, mepc, ra, t6) = (0, 0, 0, 0);
/// # unsafe {
/// *sp.sub(1) = mcause;
/// *sp.sub(2) = mepc;
/// *sp.sub(3) = ra;
/// /* ... Mem[sp - 4] ..= Mem[sp - 72] ← mcause, mepc, {xi} */
/// *sp.sub(18) = t6;
/// sp = sp.sub(18);
/// # }
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
    // ipush
    asm!(".insn i 0x0B, 0, x0, x0, 0x004")
}

/// Fast interrupt stack pop instruction.
///
/// Pop interrupt switch registers from current stack, and return from interrupt environment.
/// It pops `mcause`, `mepc`, `x1`, `x5` to `x7`, `x10` to `x17` and `x28` to `x31` from stack.
/// Another word, the popped `xi` integer registers are `ra`, `t0` to `t6`, and `a0` to `a7` (not in order)
/// other than CSR registers `mcause` and `mepc`.
///
/// In pseudocode, it performs like:
///
/// ```no_run
/// # mod riscv { pub mod asm { pub fn mret() {} }}
/// # let sp: *mut u8 = core::ptr::null_mut();
/// # unsafe {
/// let mcause = *sp.add(17);
/// let mepc = *sp.add(16);
/// let ra = *sp.add(15);
/// /* ... mcause, mepc, {xi} ← Mem[sp + 68] ..= Mem[sp] */
/// let t6 = *sp.add(0);
/// let sp = sp.add(18);
/// riscv::asm::mret();
/// # }
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
    // ipop
    asm!(".insn i 0x0B, 0, x0, x0, 0x005")
}
