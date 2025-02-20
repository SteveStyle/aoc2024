pub(crate) struct IntersectionIterator<'a> {
    pub(crate) vector1: &'a [usize],
    pub(crate) vector1_index: usize,
    pub(crate) vector2: &'a [usize],
    pub(crate) vector2_index: usize,
}

impl<'a> Iterator for IntersectionIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.vector1[self.vector1_index] != self.vector2[self.vector2_index] {
            if self.vector1[self.vector1_index] < self.vector2[self.vector2_index] {
                self.vector1_index += 1;
                if self.vector1_index == self.vector1.len() {
                    return None;
                }
            } else {
                self.vector2_index += 1;
                if self.vector2_index == self.vector2.len() {
                    return None;
                }
            }
        }
        let value = self.vector1[self.vector1_index];
        while self.vector1[self.vector1_index] == value {
            self.vector1_index += 1;
        }
        while self.vector2[self.vector2_index] == value {
            self.vector2_index += 1;
        }
        Some(value)
    }
}

impl<'a> IntersectionIterator<'a> {
    pub(crate) fn new(vector1: &'a [usize], vector2: &'a [usize]) -> Self {
        Self {
            vector1,
            vector2,
            vector1_index: 0,
            vector2_index: 0,
        }
    }
}
