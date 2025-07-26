/// Executes the given closure in a critical section.
#[inline(always)]
pub fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    const IME: *mut u32 = 0x0400_0208 as *mut u32;

    let ime = unsafe { IME.read_volatile() };

    unsafe { IME.write_volatile(0) };
    let result = f();
    unsafe { IME.write_volatile(ime) };
    
    result
}
