#![no_std]
#![feature(linkage)]

mod ipc;
mod mpu;

use core::ffi::{c_char, c_int, c_void};
use core::panic::PanicInfo;

unsafe extern "C" {
    static __sp_irq: *const c_void;
    static __sp_svc: *const c_void;
    static __sp_usr: *const c_void;

    static __bss_start: *mut u8;
    static __bss_size: usize;

    static __sbss_start: *mut u8;
    static __sbss_size: usize;

    static __preinit_array_start: *const unsafe extern "C" fn();
    static __preinit_array_size: usize;

    static __init_array_start: *const unsafe extern "C" fn();
    static __init_array_size: usize;

    static __fini_array_start: *const unsafe extern "C" fn();
    static __fini_array_size: usize;

    safe fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
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

        "mov r0, #0x1F",
        "msr cpsr_c, r0",
        "ldr sp, ={usr}",

        "b {jmp}",

        ime = const 0x0400_0208,
        irq = sym __sp_irq,
        svc = sym __sp_svc,
        usr = sym __sp_usr,
        jmp = sym _start_internal,
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start_internal() -> ! {
    #[cfg(arm9)]
    crate::mpu::init(nds::system::is_dsi());

    unsafe { core::ptr::write_bytes(__bss_start, 0, __bss_size) };
    unsafe { core::ptr::write_bytes(__sbss_start, 0, __sbss_size) };

    let _twl = crate::ipc::init();
    let _result = init_lifecycle(|| main(0, core::ptr::null()));

    nds::system::shutdown();
}

#[inline(always)]
fn init_lifecycle(main: impl FnOnce() -> i32) -> i32 {
    for &preinit in
        unsafe { core::slice::from_raw_parts(__preinit_array_start, __preinit_array_size) }
    {
        unsafe { preinit() };
    }

    for &init in unsafe { core::slice::from_raw_parts(__init_array_start, __init_array_size) } {
        unsafe { init() };
    }

    let result = main();

    for &fini in unsafe { core::slice::from_raw_parts(__fini_array_start, __fini_array_size) } {
        unsafe { fini() };
    }

    result
}

#[panic_handler]
#[linkage = "weak"]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
