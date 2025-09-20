use core::ffi::{c_int, c_longlong};

const DIV_CNT: *mut u32 = 0x0400_0280 as *mut u32;
const DIV_NUM: *mut i64 = 0x0400_0290 as *mut i64;
const DIV_DEN: *mut i64 = 0x4000_2980 as *mut i64;
const DIV_RESULT: *const i64 = 0x0400_02A0 as *const i64;
const REM_RESULT: *const i64 = 0x0400_02A8 as *const i64;
const DIV_BUSY: u32 = 1 << 15;

#[repr(u32)]
enum Mode {
    D32N32 = 0,
    D64N64 = 2,
}

/// Performs signed 32-bit integer division using the NDS hardware arithmetic unit.
///
/// # References
/// - [`__aeabi_idiv`](https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#531integer-3232--32-division-functions)
#[optimize(speed)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
extern "aapcs" fn __aeabi_idiv(num: c_int, den: c_int) -> c_int {
    unsafe { DIV_CNT.write_volatile(Mode::D32N32 as u32) };

    unsafe { (DIV_NUM as *mut i32).write_volatile(num) };
    unsafe { (DIV_DEN as *mut i32).write_volatile(den) };

    while (unsafe { DIV_CNT.read_volatile() } & DIV_BUSY) != 0 {}

    unsafe { (DIV_RESULT as *mut i32).read_volatile() }
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct divmod_t<T> {
    quot: T,
    rem: T,
}

/// Performs signed 32-bit integer division with remainder using the NDS hardware arithmetic unit.
///
/// # References
/// - [`__aeabi_idivmod`](https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#531integer-3232--32-division-functions)
#[optimize(speed)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
extern "aapcs" fn __aeabi_idivmod(num: c_int, den: c_int) -> divmod_t<c_int> {
    unsafe { DIV_CNT.write_volatile(Mode::D32N32 as u32) };

    unsafe { (DIV_NUM as *mut i32).write_volatile(num) };
    unsafe { (DIV_DEN as *mut i32).write_volatile(den) };

    while (unsafe { DIV_CNT.read_volatile() } & DIV_BUSY) != 0 {}

    divmod_t {
        quot: unsafe { (DIV_RESULT as *mut i32).read_volatile() },
        rem: unsafe { (REM_RESULT as *mut i32).read_volatile() },
    }
}

/// Performs signed 64-bit integer division with remainder using the NDS hardware arithmetic unit.
///
/// # References
/// - [`__aeabi_ldivmod`](https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#52the-long-long-helper-functions)
#[optimize(speed)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
extern "aapcs" fn __aeabi_ldivmod(num: c_longlong, den: c_longlong) -> divmod_t<c_longlong> {
    unsafe { DIV_CNT.write_volatile(Mode::D64N64 as u32) };

    unsafe { (DIV_NUM as *mut i64).write_volatile(num) };
    unsafe { (DIV_DEN as *mut i64).write_volatile(den) };

    while (unsafe { DIV_CNT.read_volatile() } & DIV_BUSY) != 0 {}

    divmod_t {
        quot: unsafe { (DIV_RESULT as *mut i64).read_volatile() },
        rem: unsafe { (REM_RESULT as *mut i64).read_volatile() },
    }
}
