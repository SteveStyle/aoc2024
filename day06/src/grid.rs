#![allow(dead_code)]

use stephen_morris_utils::pos::{Direction, Position};

#[derive(Debug, Clone)]
pub struct Grid<T: Clone + Default> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone + Default + PartialEq> Grid<T> {
    pub fn empty_with_capacity(width: usize, height: usize) -> Self {
        Self {
            data: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn new_default(width: usize, height: usize) -> Self {
        Self {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }

    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.width + col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row * self.width + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.width + col] = value;
    }


    pub fn get_pos(&self, pos: Position<usize>) -> &T {
        &self.get(pos.y, pos.x)
    }

    pub fn get_mut_pos(&mut self, pos: Position<usize>) -> &mut T {
        self.get_mut(pos.y, pos.x)
    }

    pub fn set_pos(&mut self, pos: Position<usize>, value: T) {
        self.set(pos.y, pos.x, value);
    }
    
    pub fn find(&self, value: T) -> Option<(usize, usize)> {
        for row in 0..self.height {
            for col in 0..self.width {
                if *self.get(row, col) == value {
                    return Some((row, col));
                }
            }
        }
        None
    }
    pub fn test_bounds( &self, pos: Position<usize>, direction :Direction ) -> bool {
        match direction {
            Direction::Right => pos.x < self.width - 1,
            Direction::Down => pos.y < self.height - 1,
            Direction::Left => pos.x > 0,
            Direction::Up => pos.y > 0,
            Direction::Wait => true,
        }
    }
}

impl<T: Clone + Default> From<Vec<Vec<T>>> for Grid<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        let height = v.len();
        let width = v[0].len();
        let mut data = Vec::with_capacity(width * height);
        for row in v {
            assert!(row.len() == width);
            data.extend(row);
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl<T: Clone + Default> From<&[&[T]]> for Grid<T> {
    fn from(v: &[&[T]]) -> Self {
        let height = v.len();
        let width = v[0].len();
        let mut data = Vec::with_capacity(width * height);
        for row in v {
            assert!(row.len() == width);
            data.extend(row.iter().cloned());
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl From<&str> for Grid<u8> {
    fn from(s: &str) -> Self {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut data = Vec::with_capacity(width * height);
        for line in s.lines() {
            assert!(line.len() == width);
            data.extend(line.bytes());
        }

        Self {
            data,
            width,
            height,
        }
    }
}

pub struct GridIter<T: Clone + Default> {
    grid: Grid<T>,
    row: usize,
    col: usize,
}

impl<T: Clone + Default + PartialEq> Iterator for GridIter<T> {
    type Item = (usize, usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.grid.height {
            return None;
        }

        let item = (
            self.row,
            self.col,
            self.grid.get(self.row, self.col).clone(),
        );

        self.col += 1;
        if self.col == self.grid.width {
            self.col = 0;
            self.row += 1;
        }

        Some(item)
    }
}

impl IntoIterator for Grid<u8> {
    type Item = (usize, usize, u8);
    type IntoIter = GridIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}

pub struct GridIterRef<'a, T: Clone + Default> {
    grid: &'a Grid<T>,
    row: usize,
    col: usize,
}

impl<'a> Iterator for GridIterRef<'a, u8> {
    type Item = (usize, usize, &'a u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.grid.height {
            return None;
        }

        let item = (self.row, self.col, self.grid.get(self.row, self.col));

        self.col += 1;
        if self.col == self.grid.width {
            self.col = 0;
            self.row += 1;
        }

        Some(item)
    }
}

impl<'a> IntoIterator for &'a Grid<u8> {
    type Item = (usize, usize, &'a u8);
    type IntoIter = GridIterRef<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterRef {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}






#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use crate::TESTINPUT;

    use super::*;

    #[test]
    fn test_grid() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from(v.clone());
        for (row, col, value) in grid {
            println!("{row} {col} {value}")
        }
    }
    #[test]
    fn test_grid2() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from(v.clone());
        for (row, col, value) in &grid {
            println!("{row} {col} {value}")
        }
    }
    #[test]
    fn test_grid3() {
        let grid = Grid::from(crate::TESTINPUT);
        for (row, col, value) in &grid {
            println!("{row} {col} {}", *value as char)
        }
        // print as a grid showing just the characters
    }
    #[test]
    fn test_grid4() {
        let grid = Grid::from(crate::TESTINPUT);
        // print as a grid showing just the characters
        for row in 0..grid.height {
            for col in 0..grid.width {
                print!("{}", *grid.get(row, col) as char);
            }
            println!();
        }
    }
}
