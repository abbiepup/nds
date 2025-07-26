#[cfg(arm9)]
#[inline(never)]
pub(crate) fn init(dsi: bool) {
    unsafe extern "C" {
        static __dtcm_start: u32;
        static __itcm_start: u32;
    }

    const ENABLE: u32 = 1 << 0;

    unsafe {
        // Reset cache and write buffer.
        core::arch::asm! {
            "mcr p15, #0, {zero}, c7,  c5, #0", // Flush instruction cache.
            "mcr p15, #0, {zero}, c7,  c6, #0", // Flush data cache.
            "mcr p15, #0, {zero}, c7, c10, #4", // Drain write buffer.
            zero = in(reg) 0,                   // Has to be a SBZ reg.
            options(nostack, preserves_flags),
        }

        // Set visible TCM regions. ITCM (32KB) is mirrored up to 32MB.
        core::arch::asm! {
            "mcr p15, #0, {dtcm}, c9, c1, #0", // Write DTCM.
            "mcr p15, #0, {itcm}, c9, c1, #1", // Write ITCM:
            dtcm = in(reg) 0x0080_0000 | ((TcmSize::KB16 as u32) << 1),
            itcm = in(reg) 0x0000_0000 | ((TcmSize::MB32 as u32) << 1),
            options(nostack, preserves_flags),
        }

        // Fixed MPU protection region setup.
        core::arch::asm! {
            "mcr p15, #0, {io_reg}, c6, c0, #0",
            "mcr p15, #0, {system}, c6, c1, #0",
            "mcr p15, #0, {vector}, c6, c2, #0",
            "mcr p15, #0, {dtcm},   c6, c3, #0",
            "mcr p15, #0, {itcm},   c6, c4, #0",
            io_reg = in(reg) 0x0400_0000 | RegionSize::MB64 as u32 | ENABLE,
            system = in(reg) 0xFFFF_0000 | RegionSize::KB64 as u32 | ENABLE,
            vector = in(reg) 0x0000_0000 | RegionSize::KB04 as u32 | ENABLE,
            dtcm = in(reg) __dtcm_start | RegionSize::KB16 as u32 | ENABLE,
            itcm = in(reg) (__itcm_start & !0x7FFF) | RegionSize::KB32 as u32 | ENABLE,
            options(nostack, preserves_flags),
        }

        // Platform dependent MPU region setup.
        if dsi {
        } else {
        }
    };
}

#[repr(u32)]
enum TcmSize {
    KB16 = 0b00101,
    MB32 = 0b10000,
}

#[repr(u32)]
enum RegionSize {
    KB04 = 0x0B << 1,
    KB16 = 0x0D << 1,
    KB32 = 0x0E << 1,
    KB64 = 0x0F << 1,
    MB64 = 0x19 << 1,
}
