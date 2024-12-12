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
/// *sp.sub(1) = mcause;
/// *sp.sub(2) = mepc;
/// *sp.sub(3) = ra;
/// /* ... Mem[sp - 4] ..= Mem[sp - 72] ← mcause, mepc, {xi} */
/// *sp.sub(18) = t6;
/// sp = sp.sub(18);
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
/// mcause = *sp.add(17);
/// mepc = *sp.add(16);
/// ra = *sp.add(15);
/// /* ... mcause, mepc, {xi} ← Mem[sp + 68] ..= Mem[sp] */
/// t6 = *sp.add(0);
/// sp = sp.add(18);
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
    // ipop
    asm!(".insn i 0x0B, 0, x0, x0, 0x005")
}
