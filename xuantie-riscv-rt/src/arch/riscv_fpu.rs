//! RISC-V RVF and RVD specific features.

/// Initialize RISC-V floating point module.
pub unsafe extern "C" fn init_floating_point() {
    unsafe {
        core::arch::asm! {
            "li     t0, 0x4000
            li      t1, 0x2000
            csrc    mstatus, t0
            csrs    mstatus, t1
            csrw    fcsr, zero",
        }
    }
}
