use std::{
    collections::{HashMap, HashSet},
    default,
};

use stephen_morris_utils::pos::Position;

use crate::grid::{Grid, Point, Vector};
type Cell = Option<u8>;

#[derive(Debug, Clone, PartialEq)]
pub struct AntennaLocations {
    antenna: u8,
    locations: Vec<Position<usize>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AntennaMap {
    grid: Grid<Cell>,
    antennas: HashMap<u8, Vec<Position<usize>>>,
}

pub fn parse_input(input: &str) -> AntennaMap {
    let gridu8 = Grid::from(input);
    let mut grid = Grid::<Cell>::new_default(gridu8.width, gridu8.height);
    let mut antennas = HashMap::new();
    for (x, y, c) in gridu8 {
        if c == b'.' {
            grid.set(x, y, None);
        } else {
            grid.set(x, y, Some(c));
            antennas
                .entry(c)
                .and_modify(|v: &mut Vec<Position<usize>>| v.push(Position::new(x, y)))
                .or_insert(vec![Position::new(x, y)]);
        }
    }
    AntennaMap { grid, antennas }
}

pub fn count_antinodes(antenna_map: &AntennaMap) -> usize {
    let mut antinodes: HashSet<Position<usize>> = HashSet::new();
    let grid = &antenna_map.grid;
    for (antenna, position_list) in &antenna_map.antennas {
        for i in 0..position_list.len() - 1 {
            for j in i + 1..position_list.len() {
                let i = grid.point2vec(position_list[i]).unwrap();
                let j = grid.point2vec(position_list[j]).unwrap();
                let i_side = i - (j - i);
                let j_side = j + (j - i);
                if let Some(p) = grid.vec2point(i_side) {
                    antinodes.insert(p);
                }
                if let Some(p) = grid.vec2point(j_side) {
                    antinodes.insert(p);
                }
            }
        }
    }
    antinodes.len()
}

pub fn count_antinodes2(antenna_map: &AntennaMap) -> usize {
    fn coprime(v: Vector) -> Vector {
        let gcd = num_integer::gcd(v.x, v.y);
        Vector::new(v.x / gcd, v.y / gcd)
    }
    let mut antinodes: HashSet<Position<usize>> = HashSet::new();
    let grid = &antenna_map.grid;
    for (antenna, position_list) in &antenna_map.antennas {
        for i in 0..position_list.len() - 1 {
            for j in i + 1..position_list.len() {
                let i = grid.point2vec(position_list[i]).unwrap();
                let j = grid.point2vec(position_list[j]).unwrap();
                let u = coprime(j - i);
                let mut an = i;
                while let Some(p) = grid.vec2point(an) {
                    antinodes.insert(p);
                    an = an + u;
                }
                an = i;
                while let Some(p) = grid.vec2point(an) {
                    antinodes.insert(p);
                    an = an - u;
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_antinodes() {
        let antenna_map = super::parse_input(crate::TESTINPUT);
        assert_eq!(super::count_antinodes(&antenna_map), 14);
    }

    #[test]
    fn test_count_antinodes2() {
        let antenna_map = super::parse_input(crate::TESTINPUT);
        assert_eq!(super::count_antinodes2(&antenna_map), 34);
    }
}
