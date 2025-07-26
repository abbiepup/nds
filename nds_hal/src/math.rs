use core::intrinsics::is_val_statically_known;

const DIV_CNT: *mut u16 = 0x04000_0280 as *mut u16;
const DIV_NUM: *mut i64 = 0x04000_0290 as *mut i64;
const DIV_RESULT: *mut i64 = 0x04000_02A0 as *mut i64;
// const REM_RESULT: *mut i64 = 0x0400_02A8 as *mut i64;

const DIV_32_32: u16 = 0;
const DIV_64_32: u16 = 1;
const DIV_64_64: u16 = 2;
const DIV_BUSY: u16 = 1 << 15;

#[inline(always)]
pub fn div_i32_i32(lhs: i32, rhs: i32) -> i32 {
    if is_val_statically_known(rhs) {
        return lhs / rhs;
    }

    unsafe { DIV_CNT.write_volatile(DIV_32_32) };

    unsafe {
         // Compiler didn't order regs correctly so we do it manually
        core::arch::asm! {
            "stmia r3, {{r0, r1, r2}}",
            in("r3") DIV_NUM, // Numerator base
            in("r0") lhs,     // Numerator low
            in("r1") 0,       // Numerator high
            in("r2") rhs,     // Denominator low
            options(nostack)
        }
    }

    while (unsafe { DIV_CNT.read_volatile() } & DIV_BUSY) != 0 {}

    unsafe { (DIV_RESULT as *mut i32).read_volatile() }
}

#[inline(always)]
pub fn div_i64_i32(lhs: i64, rhs: i32) -> i64 {
    if is_val_statically_known(rhs) {
        return lhs / rhs as i64;
    }

    unsafe { DIV_CNT.write_volatile(DIV_64_32) };

    unsafe {
        core::arch::asm! {
            "stmia {num}, {{{num_l}, {num_h}, {den_l}}}",
            num = in(reg) DIV_NUM,
            num_l = in(reg) lhs as u32,
            num_h = in(reg) (lhs >> 32) as u32,
            den_l = in(reg) rhs,
            options(nostack)
        }
    }

    while (unsafe { DIV_CNT.read_volatile() } & DIV_BUSY) != 0 {}

    unsafe { (DIV_RESULT).read_volatile() }
}

#[inline(always)]
pub fn div_i64_i64(lhs: i64, rhs: i64) -> i64 {
    if is_val_statically_known(rhs) {
        return lhs / rhs;
    }

    unsafe { DIV_CNT.write_volatile(DIV_64_64) };

    unsafe {
        core::arch::asm! {
            "stmia {num}, {{{num_l}, {num_h}, {den_l}, {den_h}}}",
            num = in(reg) DIV_NUM,
            num_l = in(reg) lhs as u32,
            num_h = in(reg) (lhs >> 32) as u32,
            den_l = in(reg) rhs as u32,
            den_h = in(reg) (rhs >> 32) as u32,
            options(nostack)
        }
    }

    while (unsafe { DIV_CNT.read_volatile() } & DIV_BUSY) != 0 {}

    unsafe { (DIV_RESULT).read_volatile() }
}
