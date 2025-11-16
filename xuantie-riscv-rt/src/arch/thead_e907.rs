use super::riscv_fpu::init_floating_point;
use crate::{
    main,
    stack::{STACK, STACK_SIZE},
};

#[unsafe(naked)]
#[unsafe(link_section = ".init")]
pub unsafe extern "C" fn thead_e907_start() -> ! {
    core::arch::naked_asm!(
        // 1. Disable interrupt
        "   csrw    mie, zero",
        // 2. Hart specific initialization
        // Enable T-Head instruction sets (THEADISAEE) and
        // misaligned access (MM) in `mxstatus` register.
        "   li      t1, 0x408000
            csrs    0x7c0, t1",
        // TODO SPUSHEN and SPSWAPEN in `mexstatus` once we have trap handler
        // Enable T-Head caches (BTB=1, BPE=1, RS=1, WA=1, WB=1, DE=1, IE=1)
        // in `mhcr` register;
        // enable T-Head hint operations (PREF_N=3, AMR=1, D_PLD=1)
        // in `mhint` register.
        "   li      t0, 0x103f
            csrw    0x7c1, t0
            li      t1, 0x600c
            csrw    0x7c5, t1",
        // 3. Initialize float point unit
        "   call    {init_floating_point}",
        // 4. Clear `.bss` section
        "   la      t0, sbss
            la      t1, ebss
        2:  bgeu    t0, t1, 3f
            sw      zero, 0(t0)
            addi    t0, t0, 4
            j       2b",
        "3:",
        // 5. Prepare programming language stack
        "   la      sp, {stack} + {stack_size}",
        // 6. Start Rust main function
        "   call   {main}",
        // 7. Platform halt if main function returns
        "   call   {thead_e907_halt}",
        stack      =   sym STACK,
        stack_size = const STACK_SIZE,
        init_floating_point = sym init_floating_point,
        main       =   sym main,
        thead_e907_halt = sym thead_e907_halt,
    )
}

/// Stop a T-Head E907 core.
#[unsafe(naked)]
pub unsafe extern "C" fn thead_e907_halt() -> ! {
    core::arch::naked_asm!(
        "li      t0, 0x1c
        csrc    0x7e1, t0
        csrci   mstatus, 0x8
    2:  wfi
        j       2b",
    )
}
