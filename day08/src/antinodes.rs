use std::{collections::HashMap, default};

use stephen_morris_utils::pos::Position;

use crate::grid::Grid;
type Cell = Option<u8>;

pub struct AntennaLocations {
    antenna: u8,
    locations: Vec<Position<usize>>,
}

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
