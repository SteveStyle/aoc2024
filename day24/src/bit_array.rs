#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitArray(pub u64);

impl BitArray {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn set(&mut self, index: usize) {
        self.0 |= 1 << index;
    }
    pub fn unset(&mut self, index: usize) {
        self.0 &= !(1 << index);
    }
    pub fn get(&self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }
    pub fn set_value(&mut self, index: usize, value: bool) {
        if value {
            self.set(index);
        } else {
            self.unset(index);
        }
    }
}

impl std::ops::BitAndAssign for BitArray {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl std::ops::BitOrAssign for BitArray {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl std::ops::BitXorAssign for BitArray {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
impl std::ops::ShlAssign<usize> for BitArray {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}
impl std::ops::ShrAssign<usize> for BitArray {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}
impl std::ops::AddAssign for BitArray {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl std::ops::SubAssign for BitArray {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl std::ops::BitAnd for BitArray {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitOr for BitArray {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for BitArray {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::Not for BitArray {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
impl std::ops::Shl<usize> for BitArray {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}
impl std::ops::Shr<usize> for BitArray {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}
impl std::ops::Add for BitArray {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl std::ops::Sub for BitArray {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
