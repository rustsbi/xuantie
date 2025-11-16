//! RISC-V RV32I and RV64I structures.

/// RISC-V program stack.
///
/// In standard RISC-V ABI specification, the stack grows downward and
/// the stack pointer is always kept 16-byte aligned.
#[repr(align(16))]
pub struct Stack<const N: usize>([u8; N]);

impl<const N: usize> Stack<N> {
    /// Create an empty stack.
    #[inline]
    pub const fn new() -> Stack<N> {
        Stack([0u8; N])
    }
}
