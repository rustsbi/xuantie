//! mcins, machine cache instruction register

set!(0x7D2);
clear!(0x7D2);

set_clear_csr! {
    /// Cache read request
    , set_r, clear_r, 1 << 0
}
