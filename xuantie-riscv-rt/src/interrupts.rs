//! Interrupt handling for Xuantie RISC-V interrupt handling standard.

unsafe extern "C" {
    fn SupervisorSoft();
    fn MachineSoft();
    fn SupervisorTimer();
    fn MachineTimer();
    fn SupervisorExternal();
    fn MachineExternal();
}

unsafe extern "C" {
    fn THeadPmuOverflow();
    fn THeadEccBusError();
}

pub static __CORE_INTERRUPTS: [Option<unsafe extern "C" fn()>; 18] = [
    None,
    Some(SupervisorSoft),
    None,
    Some(MachineSoft),
    None,
    Some(SupervisorTimer),
    None,
    Some(MachineTimer),
    None,
    Some(SupervisorExternal),
    None,
    Some(MachineExternal),
    None,
    Some(THeadPmuOverflow),
    None,
    None,
    Some(THeadEccBusError),
    Some(THeadPmuOverflow),
];

// TODO: distinguish different supported core interrupts between c906, c908, etc. cores.

#[unsafe(no_mangle)]
pub extern "C" fn _dispatch_core_interrupt(code: usize) {
    unsafe extern "C" {
        fn DefaultHandler();
    }
    match __CORE_INTERRUPTS.get(code) {
        Some(Some(handler)) => unsafe { handler() },
        _ => unsafe { DefaultHandler() },
    }
}
