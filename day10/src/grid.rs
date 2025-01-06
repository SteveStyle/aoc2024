#![allow(dead_code)]

use stephen_morris_utils::pos::{Direction, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T: Clone + Default + PartialEq> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

pub type Vector = Position<isize>;
pub type Point = Position<usize>;

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

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y * self.width + x] = value;
    }

    pub fn get_pos(&self, pos: Position<usize>) -> &T {
        self.get(pos.x, pos.y)
    }

    pub fn get_mut_pos(&mut self, pos: Position<usize>) -> &mut T {
        self.get_mut(pos.x, pos.y)
    }

    pub fn set_pos(&mut self, pos: Position<usize>, value: T) {
        self.set(pos.x, pos.y, value);
    }

    pub fn find(&self, value: T) -> Option<(usize, usize)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if *self.get(x, y) == value {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn vec2point(&self, vec: Vector) -> Option<Point> {
        if vec.x >= 0 && vec.x < self.width as isize && vec.y >= 0 && vec.y < self.height as isize {
            Position::<usize>::new_try_from_position(vec).ok()
        } else {
            None
        }
    }

    pub fn point2vec(&self, point: Point) -> Option<Vector> {
        Vector::new_try_from_position(point).ok()
    }

    pub fn test_bound(&self, pos: Position<usize>) -> Option<Position<usize>> {
        if pos.x < self.width && pos.y < self.height {
            Some(pos)
        } else {
            None
        }
    }

    pub fn test_bound_direction(&self, pos: Position<usize>, direction: Direction) -> bool {
        match direction {
            Direction::Right => pos.x < self.width - 1,
            Direction::Down => pos.y < self.height - 1,
            Direction::Left => pos.x > 0,
            Direction::Up => pos.y > 0,
            Direction::Wait => true,
        }
    }

    pub fn test_sum(
        &self,
        pos1: Position<usize>,
        pos2: Position<usize>,
    ) -> Option<Position<usize>> {
        if pos1.x + pos2.x < self.width && pos1.y + pos2.y < self.height {
            Some(Position::new(pos1.x + pos2.x, pos1.y + pos2.y))
        } else {
            None
        }
    }
    pub fn test_sub(
        &self,
        pos1: Position<usize>,
        pos2: Position<usize>,
    ) -> Option<Position<usize>> {
        if pos1.x >= pos2.x && pos1.y >= pos2.y {
            Some(Position::new(pos1.x - pos2.x, pos1.y - pos2.y))
        } else {
            None
        }
    }
    pub fn test_move(&self, pos: Position<usize>, x: isize, y: isize) -> Option<Position<usize>> {
        if !(0..self.width).contains(&((pos.x as isize + x) as usize))
            || !(0..self.height).contains(&((pos.y as isize + y) as usize))
        {
            return None;
        }
        let new_x = pos.x as isize + x;
        let new_y = pos.y as isize + y;
        Some(Position::new(new_x as usize, new_y as usize))
    }
}

impl<T: Clone + Default + PartialEq> From<Vec<Vec<T>>> for Grid<T> {
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

impl<T: Clone + Default + PartialEq> From<&[&[T]]> for Grid<T> {
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

impl Grid<u8> {
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", *self.get(x, y) as char);
            }
            println!();
        }
    }
}

pub struct GridIter<T: Clone + Default + PartialEq> {
    grid: Grid<T>,
    x: usize,
    y: usize,
}

impl<T: Clone + Default + PartialEq> Iterator for GridIter<T> {
    type Item = (usize, usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.grid.height {
            return None;
        }

        let item = (self.x, self.y, self.grid.get(self.x, self.y).clone());

        self.x += 1;
        if self.x == self.grid.width {
            self.x = 0;
            self.y += 1;
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
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIterRef<'a, T: Clone + Default + PartialEq> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridIterRef<'a, u8> {
    type Item = (usize, usize, &'a u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.grid.height {
            return None;
        }

        let item = (self.x, self.y, self.grid.get(self.x, self.y));

        self.x += 1;
        if self.x == self.grid.width {
            self.x = 0;
            self.y += 1;
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
            x: 0,
            y: 0,
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
        for (x, y, value) in grid {
            println!("{x} {y} {value}")
        }
    }
    #[test]
    fn test_grid2() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from(v.clone());
        for (x, y, value) in &grid {
            println!("{x} {y} {value}")
        }
    }
    #[test]
    fn test_grid3() {
        let grid = Grid::from(crate::TESTINPUT);
        for (x, y, value) in &grid {
            println!("{x} {y} {}", *value as char)
        }
    }
    #[test]
    fn test_grid4() {
        let grid = Grid::from(crate::TESTINPUT);
        // print as a grid showing just the characters
        for x in 0..grid.height {
            for y in 0..grid.width {
                print!("{}", *grid.get(x, y) as char);
            }
            println!();
        }
    }
}
