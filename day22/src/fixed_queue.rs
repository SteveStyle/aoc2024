use std::ops::Deref;

use num_traits::ConstZero;

use num_traits::Signed;

use num_traits::Num;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FixedQueue<T: Num + Signed + ConstZero + Copy, const COUNT: usize>([T; COUNT]);

impl<T: Num + Signed + ConstZero + Copy, const COUNT: usize> Deref for FixedQueue<T, COUNT> {
    type Target = [T; COUNT];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Num + Signed + ConstZero + Copy, const COUNT: usize> FixedQueue<T, COUNT> {
    pub(crate) fn new() -> Self {
        Self([T::ZERO; COUNT])
    }
    pub(crate) fn push(&mut self, new_value: T) {
        for i in 0..COUNT - 1 {
            self.0[i] = self.0[i + 1];
        }
        self.0[COUNT - 1] = new_value;
    }
    pub(crate) fn as_slice(&self) -> &[T; COUNT] {
        &self.0
    }
    pub(crate) fn delta(&self) -> T {
        self.0[COUNT - 1] - self.0[COUNT - 2]
    }
}
