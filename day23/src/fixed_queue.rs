use num_traits::{Num, Signed};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FixedQueue<T: Default + Copy, const COUNT: usize>([T; COUNT]);

impl<T: Default + Copy, const COUNT: usize> FixedQueue<T, COUNT> {
    pub(crate) fn new() -> Self {
        Self([T::default(); COUNT])
    }
    pub(crate) fn push(&mut self, new_value: T) {
        for i in (0..COUNT - 1).rev() {
            self.0[i + 1] = self.0[i];
        }
        self.0[0] = new_value;
    }
    pub(crate) fn top(&self) -> T {
        self.0[0]
    }
}

impl<T: Num + Signed + Default + Copy, const COUNT: usize> FixedQueue<T, COUNT> {
    pub(crate) fn delta(&self) -> T {
        self.0[0] - self.0[1]
    }
}

impl<T: Default + Copy, const COUNT: usize> Deref for FixedQueue<T, COUNT> {
    type Target = [T; COUNT];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default + Copy, const COUNT: usize> DerefMut for FixedQueue<T, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
