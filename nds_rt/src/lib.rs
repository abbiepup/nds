#![no_std]
#![feature(linkage)]
#![feature(asm_cfg)]
#![feature(optimize_attribute)]

mod math;

use core::ffi::{c_char, c_int, c_void};
use core::panic::PanicInfo;

unsafe extern "C" {
    /// Address of the top of the IRQ-mode stack.
    ///
    /// Defined in the linker script. Used to initialize the
    /// CPU stack pointer when handling IRQ exceptions.
    static __sp_irq: *const c_void;

    /// Address of the top of the Supervisor mode stack.
    ///
    /// Defined in the linker script.
    /// Used during system calls or other privileged operations.
    static __sp_svc: *const c_void;

    /// Address of the top of the User mode stack.
    ///
    /// Defined in the linker script. Used for unprivileged code execution.
    static __sp_usr: *const c_void;

    /// Start address of the `.bss` section.
    static __bss_start: *mut u8;

    /// Size of the `.bss` section in bytes.
    ///
    /// Used to know how many bytes to zero during startup.
    static __bss_size: usize;

    // static __sbss_start: *mut u8;
    // static __sbss_size: usize;

    static __itcm_start: *const c_void;
    static __dtcm_start: *const c_void;

    safe fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

/// Bare-metal entry point.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm! {
        "ldr r0, ={ime}",
        "mov r1, #0",
        "str r1, [r0]",

        "mov r0, #0x12",
        "msr cpsr_c, r0",
        "ldr sp, ={irq}",

        "mov r0, #0x13",
        "msr cpsr_c, r0",
        "ldr sp, ={svc}",

        #[cfg(all(target_arch = "arm", target_feature = "v5te"))]
        "bl {mpu}",

        "mov r0, #0x1F",
        "msr cpsr_c, r0",
        "ldr sp, ={usr}",

        "b {jmp}",

        ime = const 0x0400_0208,
        irq = sym __sp_irq,
        svc = sym __sp_svc,
        usr = sym __sp_usr,
        mpu = sym _start_mpu,
        jmp = sym _start_internal,
    };
}

#[cfg(all(target_arch = "arm", target_feature = "v5te"))]
#[doc(hidden)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn _start_mpu() {
    unsafe {
        core::arch::asm! {
            // Disable everything except alternate vector select.
            "mcr p15, #0, {ctrl}, c1, c0, #0", // Write control.

            // Reset cache and write buffer.
            "mcr p15, #0, {zero}, c7,  c5, #0", // Flush I-cache.
            "mcr p15, #0, {zero}, c7,  c6, #0", // Flush D-cache.
            "mcr p15, #0, {zero}, c7, c10, #4", // Drain write buffer.

            // Set visible TCM regions. ITCM (32KB) is mirrored up to 32MB.
            "mcr p15, #0, {itcm}, c9, c1, #1", // Write ITCM.
            "mcr p15, #0, {dtcm}, c9, c1, #0", // Write DTCM.

            ctrl = in(reg) (1 << 13) | 0b0111_1000,    // Alternate vector select.
            zero = in(reg) 0,                          // SBZ
            itcm = in(reg) 0x0000_0000 | 0b10000 << 1, // 32MB
            dtcm = in(reg) 0x0080_0000 | 0b00101 << 1, // 16KB
            options(nostack, preserves_flags),
        }
    };
}

#[doc(hidden)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
unsafe extern "C" fn _start_internal() -> ! {
    unsafe { core::ptr::write_bytes(__bss_start, 0, __bss_size) };
    // unsafe { core::ptr::write_bytes(__sbss_start, 0, __sbss_size) };

    let _result = main(0, core::ptr::null());

    loop {}
}

#[cold]
#[panic_handler]
#[linkage = "weak"]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
