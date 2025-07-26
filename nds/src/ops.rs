pub trait Div<Rhs = Self> {
    type Output;

    fn div(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_div {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Div for $t {
                type Output = $t;

                #[inline(always)]
                fn div(self, rhs: Self) -> Self::Output {
                    impl_div!(@call self, rhs, $t)
                }
            }
        )+
    };

    (@call $self:ident, $rhs:ident, i64) => {
        ::nds_hal::math::div_i64_i64($self, $rhs)
    };

    (@call $self:ident, $rhs:ident, $t:ty) => {
        ::nds_hal::math::div_i32_i32($self as i32, $rhs as i32) as $t
    };
}

impl_div!(i16, i32, i64);
