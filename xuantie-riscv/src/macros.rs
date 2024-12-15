macro_rules! read_csr_as {
    ($register:ident, $csr_number:expr) => {
        /// Reads the CSR
        #[inline]
        pub fn read() -> $register {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => {
                    let bits: usize;
                    unsafe { core::arch::asm!(concat!("csrr {}, ",$csr_number), out(reg) bits) };
                    $register { bits }
                }
                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => {
                    unimplemented!()
                }
            }
        }
    };
}

macro_rules! set_csr {
    ($(#[$attr:meta])*, $csr_number:expr, $set_field:ident, $e:expr) => {
        $(#[$attr])*
        #[inline]
        pub unsafe fn $set_field() {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => {
                    core::arch::asm!(concat!("csrs ",$csr_number,", {0}"), in(reg) $e)
                }
                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => {
                    unimplemented!()
                }
            }
        }
    }
}

macro_rules! clear_csr {
    ($(#[$attr:meta])*, $csr_number:expr, $clear_field:ident, $e:expr) => {
        $(#[$attr])*
        #[inline]
        pub unsafe fn $clear_field() {
            match () {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                () => {
                    core::arch::asm!(concat!("csrc ",$csr_number,", {0}"), in(reg) $e)
                }
                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                () => {
                    unimplemented!()
                }
            }
        }
    }
}

macro_rules! set_clear_csr {
    ($(#[$attr:meta])*, $csr_number:expr, $set_field:ident, $clear_field:ident, $e:expr) => {
        set_csr!($(#[$attr])*, $csr_number, $set_field, $e);
        clear_csr!($(#[$attr])*, $csr_number, $clear_field, $e);
    }
}
macro_rules! get_csr_value {
    ($csr_number:expr) => {
        match () {
            #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
            () => {
                let r: usize;
                core::arch::asm!(concat!("csrr {}, ",$csr_number), out(reg) r);
                r
            }
            #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
            () => {
                unimplemented!()
            }
        }
    };
}
