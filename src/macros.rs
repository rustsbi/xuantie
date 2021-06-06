macro_rules! read_csr {
    ($csr_number:expr) => {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            let r: usize;
            asm!("csrr {}, {}", out(reg) r, const $csr_number);
            r
        }
    };
}

macro_rules! read_csr_as {
    ($register:ident, $csr_number:expr) => {
        read_csr!($csr_number);

        /// Reads the CSR
        #[inline]
        pub fn read() -> $register {
            $register {
                bits: unsafe { _read() },
            }
        }
    };
}

macro_rules! set {
    ($csr_number:expr) => {
        /// Set the CSR
        #[inline]
        unsafe fn _set(bits: usize) {
            asm!("csrs {1}, {0}", in(reg) bits, const $csr_number)
        }
    };
}

macro_rules! clear {
    ($csr_number:expr) => {
        /// Clear the CSR
        #[inline]
        unsafe fn _clear(bits: usize) {
            asm!("csrc {1}, {0}", in(reg) bits, const $csr_number)
        }
    };
}

macro_rules! set_csr {
    ($(#[$attr:meta])*, $set_field:ident, $e:expr) => {
        $(#[$attr])*
        #[inline]
        pub unsafe fn $set_field() {
            _set($e);
        }
    }
}

macro_rules! clear_csr {
    ($(#[$attr:meta])*, $clear_field:ident, $e:expr) => {
        $(#[$attr])*
        #[inline]
        pub unsafe fn $clear_field() {
            _clear($e);
        }
    }
}

macro_rules! set_clear_csr {
    ($(#[$attr:meta])*, $set_field:ident, $clear_field:ident, $e:expr) => {
        set_csr!($(#[$attr])*, $set_field, $e);
        clear_csr!($(#[$attr])*, $clear_field, $e);
    }
}
macro_rules! get_csr_value {
    ($csr_number:expr) => {
        {
            let r: usize;
            asm!("csrr {}, {}", out(reg) r, const $csr_number);
            r
        }
    };
}