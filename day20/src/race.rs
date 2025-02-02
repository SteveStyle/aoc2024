use std::{collections::HashMap, ops::Deref};

use crate::grid::{Direction, Grid, Point, Vector};

type Count = usize;

#[derive(Debug)]
struct Cheat {
    start: Point,
    end: Point,
    saving: Count,
}

impl Deref for Cheat {
    type Target = Count;

    fn deref(&self) -> &Self::Target {
        &self.saving
    }
}

impl PartialEq for Cheat {
    fn eq(&self, other: &Self) -> bool {
        self.saving == other.saving
    }
}

impl Eq for Cheat {}

impl PartialOrd for Cheat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cheat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.saving.cmp(&other.saving)
    }
}

impl PartialEq<Count> for Cheat {
    fn eq(&self, other: &Count) -> bool {
        self.saving == *other
    }
}

impl PartialOrd<Count> for Cheat {
    fn partial_cmp(&self, other: &Count) -> Option<std::cmp::Ordering> {
        Some(self.saving.cmp(other))
    }
}

#[derive(Debug)]
pub struct Race {
    grid: Grid<u8>,
    start: Point,
    end: Point,
    track: Grid<Option<Count>>,
    path: Vec<Point>,
    cheats: Vec<Cheat>,
}

impl Race {
    pub fn new(input: &str) -> Self {
        let grid = Grid::from(input);
        let start = grid.find(b'S').unwrap();
        let end = grid.find(b'E').unwrap();
        let mut track = Grid::new_default(grid.width, grid.height);
        let mut path = Vec::new();
        let cheats = Vec::new();

        let mut current_pos = start;
        let mut current_step = 0;
        track[start] = Some(0);
        path.push(current_pos);

        while current_pos != end {
            for (next_pos, &value) in grid.orthogonal_neighbors(current_pos) {
                if (value == b'.' || value == b'E') && track[next_pos].is_none() {
                    current_pos = next_pos;
                    break;
                }
            }
            current_step += 1;
            track[current_pos] = Some(current_step);
            path.push(current_pos);
        }

        Self {
            grid,
            start,
            end,
            track,
            path,
            cheats,
        }
    }

    pub fn find_cheats(&mut self) {
        for (point, &value) in &self.grid {
            if value == b'#' {
                if let (Some(east), Some(west)) = (
                    self.grid.add_direction(point, Direction::East),
                    self.grid.add_direction(point, Direction::West),
                ) {
                    if let (Some(v1), Some(v2)) = (self.track[east], self.track[west]) {
                        if v1 < v2 {
                            self.cheats.push(Cheat {
                                start: east,
                                end: west,
                                saving: v2 - v1 - 2,
                            });
                        } else {
                            self.cheats.push(Cheat {
                                start: west,
                                end: east,
                                saving: v1 - v2 - 2,
                            });
                        }
                    }
                }
                if let (Some(north), Some(south)) = (
                    self.grid.add_direction(point, Direction::North),
                    self.grid.add_direction(point, Direction::South),
                ) {
                    if let (Some(v1), Some(v2)) = (self.track[north], self.track[south]) {
                        if v1 < v2 {
                            self.cheats.push(Cheat {
                                start: north,
                                end: south,
                                saving: v2 - v1 - 2,
                            });
                        } else {
                            self.cheats.push(Cheat {
                                start: south,
                                end: north,
                                saving: v1 - v2 - 2,
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn count_cheats_over(&self, threshhold: Count) -> Count {
        self.cheats.iter().filter(|v| **v >= threshhold).count()
    }

    pub fn count_long_cheats(&self, threshhold: Count) -> Count {
        // <saving, count>
        let mut hm: HashMap<Count, Count> = HashMap::new();
        for path_index1 in 0..self.path.len() - 1 {
            for path_index2 in path_index1 + 1..self.path.len() {
                let steps = (path_index2 - path_index1);
                let manhattan = self.path[path_index1].manhattan(&self.path[path_index2]);
                if manhattan <= 20 && steps > manhattan {
                    let saving = steps - manhattan;
                    *hm.entry(saving).or_insert(0) += 1;
                }
            }
        }
        // hm.values().sum()
        hm.iter()
            .filter_map(|(&k, &v)| if k >= threshhold { Some(v) } else { None })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{race::*, *};

    #[test]
    fn test_new() {
        let race = Race::new(TESTINPUT);

        assert_eq!(race.track[race.start], Some(0));
        assert_eq!(race.track[race.end], Some(84));

        assert_eq!(race.count_long_cheats(0), 3081);
        assert_eq!(race.count_long_cheats(71), 29);
        assert_eq!(race.count_long_cheats(72), 29);
        assert_eq!(race.count_long_cheats(73), 7);

        // let hm = race.count_long_cheats();
        // // println!("total long cheats {}", hm.values().cloned().sum::<usize>());
        // println!("total long cheats {}", hm.values().sum::<usize>());
        // println!();
        // let mut vec: Vec<(Count, Count)> = hm.iter().map(|(&k, &v)| (k, v)).collect();
        // vec.sort();
        // println!("{:#?}", vec);
    }
}
