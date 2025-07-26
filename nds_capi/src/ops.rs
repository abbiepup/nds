use nds::ops::Div;

#[unsafe(no_mangle)]
pub extern "C" fn div_int16_t(lhs: i16, rhs: i16) -> i16 {
    i16::div(lhs, rhs)
}

#[unsafe(no_mangle)]
pub extern "C" fn div_int32_t(lhs: i32, rhs: i32) -> i32 {
    i32::div(lhs, rhs)
}

#[unsafe(no_mangle)]
pub extern "C" fn div_int64_t(lhs: i64, rhs: i64) -> i64 {
    i64::div(lhs, rhs)
}
