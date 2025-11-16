//! RISC-V RV32E and RV64E structures.

/// RISC-V program stack.
///
/// RV32E stacks are 4-byte aligned.
#[repr(align(4))]
pub struct Stack<const N: usize>([u8; N]);

impl<const N: usize> Stack<N> {
    /// Create an empty stack.
    #[inline]
    pub const fn new() -> Stack<N> {
        Stack([0u8; N])
    }
}
