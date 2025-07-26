#[inline]
pub fn shutdown() -> ! {
    const POWERCNT: *mut u16 = 0x0400_0304 as *mut u16;
    unsafe { POWERCNT.write_volatile(1 << 6) };
    loop {}
}
