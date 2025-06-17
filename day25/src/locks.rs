use stephen_morris_utils::bit_array::BitFlags;
use stephen_morris_utils::grid::{Grid, Point};

#[derive(Debug)]
struct Key(BitFlags<u32>);
#[derive(Debug)]
struct Lock(BitFlags<u32>);

#[derive(Debug)]
pub struct Schematics {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

impl Schematics {
    pub fn parse_input(input: &str) -> Schematics {
        let mut keys = Vec::new();
        let mut locks = Vec::new();

        let blocks = input.split("\n\n");
        for block in blocks {
            let grid = Grid::from(block);

            let mut bits: BitFlags<u32> = BitFlags::new();
            // We only care about the middle rows, so take a sub-slice.
            // i will start at zero for the first element in the sub-slice.
            for (i, char) in grid.as_slice()[5..30].iter().enumerate() {
                // bits.set_value(i, *char == b'#');
                if *char == b'#' {
                    bits.set(i)
                };
            }

            if grid[Point { x: 0, y: 0 }] == b'#' {
                locks.push(Lock(bits))
            } else {
                keys.push(Key(bits))
            }
        }

        Schematics { keys, locks }
    }

    pub fn count_matches(&self) -> usize {
        let mut count = 0;

        for lock in &self.locks {
            for key in &self.keys {
                count += ((lock.0 & key.0).0 == 0) as usize;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::{TESTINPUT, locks::Schematics};

    #[test]
    fn test_part1() {
        let count = Schematics::parse_input(TESTINPUT).count_matches();
        assert_eq!(count, 3);
    }
}
