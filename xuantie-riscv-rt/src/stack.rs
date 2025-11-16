use crate::arch::riscv_i::Stack;

pub const STACK_SIZE: usize = 8 * 1024;

#[unsafe(link_section = ".bss.uninit")]
pub static mut STACK: Stack<STACK_SIZE> = Stack::new();
