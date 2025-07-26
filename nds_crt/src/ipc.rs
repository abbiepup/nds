#[cfg(arm7)]
#[inline(always)]
pub(crate) fn init() -> bool {
    const IPC_SYNC: *mut u32 = 0x0400_0180 as *mut u32;

    for &(read, write) in &[(0x0, 0x9), (0xA, 0xB), (0xC, 0xD)] {
        while unsafe { IPC_SYNC.read_volatile() } & 0xF != read {}
        unsafe { IPC_SYNC.write_volatile(write << 8) };
    }

    loop {
        let twl = unsafe { IPC_SYNC.read_volatile() } & 0xF;
        if twl != 0xC {
            return twl == 1;
        }
    }
}

#[cfg(arm9)]
#[inline(always)]
pub(crate) fn init() -> bool {
    todo!()
}
