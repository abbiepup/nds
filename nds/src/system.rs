#[inline]
pub fn shutdown() -> ! {
    const POWERCNT: *mut u16 = 0x0400_0304 as *mut u16;
    unsafe { POWERCNT.write_volatile(1 << 6) };
    loop {}
}

// FIXME: This would only work on the arm9 atm.
#[inline]
pub fn is_dsi() -> bool {
    const SCFG_A9ROM: *const u8 = 0x0400_4000 as *const u8;
    (unsafe { SCFG_A9ROM.read_volatile() } & 0x3) == 0x1
}
