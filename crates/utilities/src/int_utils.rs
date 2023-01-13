use num_bigint::{BigInt, BigUint};
use num_integer::Integer;

pub trait DivExt {
    fn div_round_down(self, o: Self) -> Self;
    fn div_round_up(self, o: Self) -> Self;
}

macro_rules! std_div_macros {
    ($($t:ty),+) => {
        $(
            impl DivExt for $t {
                fn div_round_down(self, o: Self) -> Self {
                    (self as f32 / o as f32).floor() as $t
                }

                fn div_round_up(self, o: Self) -> Self {
                    (self as f32 / o as f32).ceil() as $t
                }
            }
        )+
    };
}
macro_rules! big_div_macros {
    ($($t:ty),+) => {
        $(
            impl DivExt for $t {
                fn div_round_down(self, o: Self) -> Self {
                    self.div_floor(&o)
                }

                fn div_round_up(self, o: Self) -> Self {
                    self.div_ceil(&o)
                }
            }
        )+
    };
}

std_div_macros!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
big_div_macros!(BigUint, BigInt);
