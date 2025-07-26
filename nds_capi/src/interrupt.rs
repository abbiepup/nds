use core::ffi::c_void;

/// Executes the given closure in a critical section.
#[unsafe(no_mangle)]
pub extern "C" fn critical_section(
    f: extern "C" fn(*mut c_void) -> *mut c_void,
    ctx: *mut c_void,
) -> *mut c_void {
    nds_hal::interrupt::critical_section(|| f(ctx))
}
