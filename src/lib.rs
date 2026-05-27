#![no_std]

use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Fixed<T, const FRAC: u32> {
    pub bits: T,
}

impl<T: Copy, const FRAC: u32> Fixed<T, FRAC> {
    #[inline(always)]
    pub const fn from_bits(bits: T) -> Self {
        Self { bits }
    }

    #[inline(always)]
    pub const fn to_bits(self) -> T {
        self.bits
    }
}

macro_rules! impl_common_traits {
    ($t:ty) => {
        impl<const FRAC: u32> Add for Fixed<$t, FRAC> {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                self.add_unchecked(rhs)
            }
        }
        impl<const FRAC: u32> Sub for Fixed<$t, FRAC> {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                self.sub_unchecked(rhs)
            }
        }
        impl<const FRAC: u32> Mul for Fixed<$t, FRAC> {
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: Self) -> Self::Output {
                self.mul_blaze(rhs)
            }
        }
        impl<const FRAC: u32> Div for Fixed<$t, FRAC> {
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                self.div_blaze(rhs)
            }
        }
        impl<const FRAC: u32> Rem for Fixed<$t, FRAC> {
            type Output = Self;
            #[inline(always)]
            fn rem(self, rhs: Self) -> Self::Output {
                self.rem_blaze(rhs)
            }
        }
        impl<const FRAC: u32> Shl<u32> for Fixed<$t, FRAC> {
            type Output = Self;
            #[inline(always)]
            fn shl(self, rhs: u32) -> Self::Output {
                Self::from_bits(self.bits << rhs)
            }
        }
        impl<const FRAC: u32> Shr<u32> for Fixed<$t, FRAC> {
            type Output = Self;
            #[inline(always)]
            fn shr(self, rhs: u32) -> Self::Output {
                Self::from_bits(self.bits >> rhs)
            }
        }
        impl<const FRAC: u32> AddAssign for Fixed<$t, FRAC> {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl<const FRAC: u32> SubAssign for Fixed<$t, FRAC> {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl<const FRAC: u32> MulAssign for Fixed<$t, FRAC> {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
        impl<const FRAC: u32> DivAssign for Fixed<$t, FRAC> {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
        impl<const FRAC: u32> RemAssign for Fixed<$t, FRAC> {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }
        impl<const FRAC: u32> ShlAssign<u32> for Fixed<$t, FRAC> {
            #[inline(always)]
            fn shl_assign(&mut self, rhs: u32) {
                *self = *self << rhs;
            }
        }
        impl<const FRAC: u32> ShrAssign<u32> for Fixed<$t, FRAC> {
            #[inline(always)]
            fn shr_assign(&mut self, rhs: u32) {
                *self = *self >> rhs;
            }
        }
    };
}

macro_rules! impl_fixed_ops {
    (unsigned $t:ty, $wide:ty) => {
        impl<const FRAC: u32> Fixed<$t, FRAC> {
            pub const ZERO: Self = Self::from_bits(0);
            pub const MIN: Self = Self::from_bits(<$t>::MIN);
            pub const MAX: Self = Self::from_bits(<$t>::MAX);

            #[inline(always)]
            pub const fn is_zero(self) -> bool {
                self.bits == 0
            }

            #[inline(always)]
            pub const fn rem_blaze(self, rhs: Self) -> Self {
                if rhs.bits == 0 {
                    panic!("Remainder by zero in fixed-point arithmetic!");
                }
                Self::from_bits(self.bits % rhs.bits)
            }

            #[inline(always)]
            pub const fn floor(self) -> Self {
                if FRAC >= core::mem::size_of::<$t>() as u32 * 8 {
                    return Self::ZERO;
                }
                if FRAC == 0 {
                    return self;
                }
                let mask = !(((1 as $t) << FRAC) - 1);
                Self::from_bits(self.bits & mask)
            }

            #[inline(always)]
            pub const fn ceil(self) -> Self {
                if FRAC >= core::mem::size_of::<$t>() as u32 * 8 {
                    return if self.bits > 0 {
                        Self::from_bits(1)
                    } else {
                        Self::ZERO
                    };
                }
                let f = self.floor();
                if f.bits == self.bits {
                    f
                } else {
                    Self::from_bits(f.bits.wrapping_add((1 as $t) << FRAC))
                }
            }

            #[inline(always)]
            pub const fn round(self) -> Self {
                if FRAC >= core::mem::size_of::<$t>() as u32 * 8 {
                    return Self::ZERO;
                }
                if FRAC == 0 {
                    return self;
                }
                let half = (1 as $t) << (FRAC - 1);
                Self::from_bits(self.bits.wrapping_add(half)).floor()
            }

            #[inline(always)]
            pub const fn div_blaze(self, rhs: Self) -> Self {
                let num = self.bits as $wide;
                let den = rhs.bits as $wide;
                if den == 0 {
                    panic!("Division by zero in fixed-point arithmetic!");
                }

                let temp_num = num << FRAC;
                let mut quot: $wide = 0;
                let mut rem: $wide = 0;
                let total_bits = core::mem::size_of::<$wide>() * 8;

                let mut i = total_bits;
                while i > 0 {
                    i -= 1;
                    rem = (rem << 1) | ((temp_num >> i) & 1);
                    if rem >= den {
                        rem -= den;
                        quot |= 1 << i;
                    }
                }

                let round_up = if rem * 2 > den {
                    true
                } else if rem * 2 == den {
                    (quot & 1) != 0
                } else {
                    false
                };
                if round_up {
                    quot += 1;
                }

                Self::from_bits(quot as $t)
            }

            #[inline(always)]
            pub const fn mul_blaze(self, rhs: Self) -> Self {
                if FRAC == 0 {
                    return Self::from_bits(self.bits.wrapping_mul(rhs.bits));
                }
                let a = self.bits as $wide;
                let b = rhs.bits as $wide;
                let wide_prod = a * b;

                let safe_frac = if FRAC > 0 { FRAC } else { 1 };
                let half = (1 as $wide) << (safe_frac - 1);
                let frac_mask = ((1 as $wide) << safe_frac) - 1;

                let shifted = wide_prod >> safe_frac;
                let fraction = wide_prod & frac_mask;

                let rounded = if fraction > half {
                    shifted + 1
                } else if fraction < half {
                    shifted
                } else {
                    if (shifted & 1) != 0 {
                        shifted + 1
                    } else {
                        shifted
                    }
                };
                Self::from_bits(rounded as $t)
            }

            #[inline(always)]
            pub const fn add_unchecked(self, rhs: Self) -> Self {
                Self::from_bits(self.bits.wrapping_add(rhs.bits))
            }
            #[inline(always)]
            pub const fn sub_unchecked(self, rhs: Self) -> Self {
                Self::from_bits(self.bits.wrapping_sub(rhs.bits))
            }
            #[inline(always)]
            pub const fn min(self, other: Self) -> Self {
                if self.bits < other.bits {
                    self
                } else {
                    other
                }
            }
            #[inline(always)]
            pub const fn max(self, other: Self) -> Self {
                if self.bits > other.bits {
                    self
                } else {
                    other
                }
            }
        }

        impl_common_traits!($t);
    };

    (signed $t:ty, $wide:ty, $uwide:ty) => {
        impl<const FRAC: u32> Fixed<$t, FRAC> {
            pub const ZERO: Self = Self::from_bits(0);
            pub const MIN: Self = Self::from_bits(<$t>::MIN);
            pub const MAX: Self = Self::from_bits(<$t>::MAX);

            #[inline(always)]
            pub const fn is_zero(self) -> bool {
                self.bits == 0
            }
            #[inline(always)]
            pub const fn is_positive(self) -> bool {
                self.bits > 0
            }
            #[inline(always)]
            pub const fn is_negative(self) -> bool {
                self.bits < 0
            }

            #[inline(always)]
            pub const fn abs(self) -> Self {
                Self::from_bits(if self.bits < 0 {
                    self.bits.wrapping_neg()
                } else {
                    self.bits
                })
            }

            #[inline(always)]
            pub const fn rem_blaze(self, rhs: Self) -> Self {
                if rhs.bits == 0 {
                    panic!("Remainder by zero in fixed-point arithmetic!");
                }
                Self::from_bits(self.bits % rhs.bits)
            }

            #[inline(always)]
            pub const fn floor(self) -> Self {
                if FRAC >= core::mem::size_of::<$t>() as u32 * 8 {
                    return if self.is_negative() {
                        Self::from_bits(-1)
                    } else {
                        Self::ZERO
                    };
                }
                if FRAC == 0 {
                    return self;
                }
                let mask = !(((1 as $t) << FRAC) - 1);

                Self::from_bits(self.bits & mask)
            }

            #[inline(always)]
            pub const fn ceil(self) -> Self {
                if FRAC >= core::mem::size_of::<$t>() as u32 * 8 {
                    return if self.is_positive() {
                        Self::from_bits(1)
                    } else {
                        Self::ZERO
                    };
                }
                let f = self.floor();
                if f.bits == self.bits {
                    f
                } else {
                    Self::from_bits(f.bits.wrapping_add((1 as $t) << FRAC))
                }
            }

            #[inline(always)]
            pub const fn round(self) -> Self {
                if FRAC >= core::mem::size_of::<$t>() as u32 * 8 {
                    return self.floor();
                }
                if FRAC == 0 {
                    return self;
                }
                let half = (1 as $t) << (FRAC - 1);
                Self::from_bits(self.bits.wrapping_add(half)).floor()
            }

            #[inline(always)]
            pub const fn div_blaze(self, rhs: Self) -> Self {
                let neg = (self.bits < 0) ^ (rhs.bits < 0);
                let num = self.bits.unsigned_abs() as $uwide;
                let den = rhs.bits.unsigned_abs() as $uwide;
                if den == 0 {
                    panic!("Division by zero in fixed-point arithmetic!");
                }

                let temp_num = num << FRAC;
                let mut quot: $uwide = 0;
                let mut rem: $uwide = 0;
                let total_bits = core::mem::size_of::<$uwide>() * 8;

                let mut i = total_bits;
                while i > 0 {
                    i -= 1;
                    rem = (rem << 1) | ((temp_num >> i) & 1);
                    if rem >= den {
                        rem -= den;
                        quot |= 1 << i;
                    }
                }

                let round_up = if rem * 2 > den {
                    true
                } else if rem * 2 == den {
                    (quot & 1) != 0
                } else {
                    false
                };
                if round_up {
                    quot += 1;
                }

                let result_bits = quot as $t;
                Self::from_bits(if neg {
                    result_bits.wrapping_neg()
                } else {
                    result_bits
                })
            }

            #[inline(always)]
            pub const fn mul_blaze(self, rhs: Self) -> Self {
                if FRAC == 0 {
                    return Self::from_bits(self.bits.wrapping_mul(rhs.bits));
                }
                let a = self.bits as $wide;
                let b = rhs.bits as $wide;
                let wide_prod = a * b;

                let safe_frac = if FRAC > 0 { FRAC } else { 1 };
                let half = (1 as $wide) << (safe_frac - 1);
                let frac_mask = ((1 as $wide) << safe_frac) - 1;

                let shifted = wide_prod >> safe_frac;
                let fraction = wide_prod & frac_mask;

                let rounded = if fraction > half {
                    shifted + 1
                } else if fraction < half {
                    shifted
                } else {
                    if (shifted & 1) != 0 {
                        shifted + 1
                    } else {
                        shifted
                    }
                };
                Self::from_bits(rounded as $t)
            }

            #[inline(always)]
            pub const fn add_unchecked(self, rhs: Self) -> Self {
                Self::from_bits(self.bits.wrapping_add(rhs.bits))
            }
            #[inline(always)]
            pub const fn sub_unchecked(self, rhs: Self) -> Self {
                Self::from_bits(self.bits.wrapping_sub(rhs.bits))
            }
            #[inline(always)]
            pub const fn min(self, other: Self) -> Self {
                if self.bits < other.bits {
                    self
                } else {
                    other
                }
            }
            #[inline(always)]
            pub const fn max(self, other: Self) -> Self {
                if self.bits > other.bits {
                    self
                } else {
                    other
                }
            }
        }

        impl_common_traits!($t);

        impl<const FRAC: u32> Neg for Fixed<$t, FRAC> {
            type Output = Self;
            #[inline(always)]
            fn neg(self) -> Self::Output {
                Self::from_bits(self.bits.wrapping_neg())
            }
        }
    };
}

impl_fixed_ops!(unsigned u64, u128);
impl_fixed_ops!(signed i32, i64, u64);
impl_fixed_ops!(signed i64, i128, u128);
