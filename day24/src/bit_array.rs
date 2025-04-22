use std::{
    default,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOrAssign, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
};

use num_traits::PrimInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BitArray<T: PrimInt + BitOrAssign>(pub T);

impl<T: PrimInt + BitOrAssign + BitAndAssign> BitArray<T> {
    pub fn new() -> Self {
        Self(T::zero())
    }
    pub fn set(&mut self, index: usize) {
        self.0 |= T::one() << index;
    }
    pub fn unset(&mut self, index: usize) {
        self.0 &= !(T::one() << index);
    }
    pub fn get(&self, index: usize) -> bool {
        (self.0 & (T::one() << index)) != T::zero()
    }
    pub fn set_value(&mut self, index: usize, value: bool) {
        if value {
            self.set(index);
        } else {
            self.unset(index);
        }
    }
}

impl<T: PrimInt + BitOrAssign + BitAndAssign> std::ops::BitAndAssign for BitArray<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> std::ops::BitOrAssign for BitArray<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl<T: PrimInt + BitXorAssign + BitOrAssign> std::ops::BitXorAssign for BitArray<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
impl<T: PrimInt + ShlAssign<usize> + BitOrAssign> ShlAssign<usize> for BitArray<T> {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}
impl<T: PrimInt + BitOrAssign + ShrAssign<usize>> ShrAssign<usize> for BitArray<T> {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}
impl<T: PrimInt + BitOrAssign + AddAssign> AddAssign for BitArray<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl<T: PrimInt + BitOrAssign + SubAssign> SubAssign for BitArray<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> BitAnd for BitArray<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> std::ops::BitOr for BitArray<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> std::ops::BitXor for BitArray<T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Not for BitArray<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Shl<usize> for BitArray<T> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Shr<usize> for BitArray<T> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Add for BitArray<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Sub for BitArray<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
