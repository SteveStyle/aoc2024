const TESTINPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

use crate::grid;
use crate::grid::{Grid, Point, Vector};
use stephen_morris_utils::get_numbers;

#[cfg(test)]
mod config {
    pub const WIDTH: usize = 11;
    pub const HEIGHT: usize = 7;
}
#[cfg(not(test))]
mod config {
    pub const WIDTH: usize = 101;
    pub const HEIGHT: usize = 103;
}

use config::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Robot {
    position: Point,
    velocity: Vector,
}

impl Robot {
    pub fn move_robot(&mut self, seconds: usize) -> &Self {
        let mut position = Vector::from(self.position);
        position += self.velocity * seconds as isize;
        position.x = position.x.rem_euclid(WIDTH as isize);
        position.y = position.y.rem_euclid(HEIGHT as isize);
        self.position = Point::from(position);
        self
    }
    pub fn quadrant(&self) -> Option<usize> {
        if self.position.x < WIDTH / 2 && self.position.y < HEIGHT / 2 {
            Some(1)
        } else if self.position.x > WIDTH / 2 && self.position.y < HEIGHT / 2 {
            Some(2)
        } else if self.position.x < WIDTH / 2 && self.position.y > HEIGHT / 2 {
            Some(3)
        } else if self.position.x > WIDTH / 2 && self.position.y > HEIGHT / 2 {
            Some(4)
        } else {
            None
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let v = get_numbers(line);

            Robot {
                position: Point {
                    x: v[0] as usize,
                    y: v[1] as usize,
                },
                velocity: Vector { x: v[2], y: v[3] },
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct Restroom {
    robots: Vec<Robot>,
}

impl Restroom {
    pub fn new(input: &str) -> Self {
        Restroom {
            robots: parse_input(input),
        }
    }

    pub fn move_robots(&mut self, seconds: usize) -> &Self {
        self.robots.iter_mut().for_each(|robot| {
            robot.move_robot(seconds);
        });
        self
    }

    pub fn safety_factor(&self) -> usize {
        let mut counts = [0; 4];
        self.robots
            .iter()
            .filter_map(|robot| robot.quadrant())
            .for_each(|quadrant| counts[quadrant - 1] += 1);
        counts.iter().product()
    }

    pub fn safety_factor_at_time(&self, seconds: usize) -> usize {
        let mut restroom = self.clone();
        restroom.move_robots(seconds);
        restroom.safety_factor()
    }

    pub fn minimize_safety_factor(&mut self, max_seconds: usize) -> usize {
        let mut seconds = 0;
        let mut min_safety_factor = self.safety_factor();
        let mut min_safety_factor_time = 0;
        let mut restroom = self.clone();
        while min_safety_factor > 0 && seconds < max_seconds {
            let safety_factor = restroom.safety_factor();
            if safety_factor < min_safety_factor {
                min_safety_factor = safety_factor;
                min_safety_factor_time = seconds;
            }
            restroom.move_robots(1);
            seconds += 1;
        }
        min_safety_factor_time
    }

    pub fn print(&self) {
        let mut grid: grid::Grid<u8> = grid::Grid::new(WIDTH, HEIGHT, b' ');
        for robot in &self.robots {
            grid.set(robot.position, b'*');
        }
        grid.print();
    }

    pub fn print_at_minimum_safety_factor(&self, max_seconds: usize) -> usize {
        let mut restroom = self.clone();
        let seconds = restroom.minimize_safety_factor(max_seconds);
        restroom.move_robots(seconds).print();
        seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_safety_factor() {
        let mut restroom = Restroom::new(TESTINPUT);
        restroom.print_at_minimum_safety_factor(1000);
        //   assert_eq!(restroom.minimize_safety_factor(1000), 3);
    }

    #[test]
    fn test_parse_input() {
        let robots = parse_input(TESTINPUT);
        assert_eq!(robots.len(), 12);
        assert_eq!(
            robots[0],
            Robot {
                position: Point { x: 0, y: 4 },
                velocity: Vector { x: 3, y: -3 }
            }
        );
        assert_eq!(
            robots[11],
            Robot {
                position: Point { x: 9, y: 5 },
                velocity: Vector { x: -3, y: -3 }
            }
        );
    }

    #[test]
    fn test_move_robot() {
        let mut robot = Robot {
            position: Point { x: 0, y: 0 },
            velocity: Vector { x: 1, y: 1 },
        };
        robot.move_robot(1);
        assert_eq!(robot.position, Point { x: 1, y: 1 });
        robot.move_robot(1);
        assert_eq!(robot.position, Point { x: 2, y: 2 });
        robot.move_robot(1);
        assert_eq!(robot.position, Point { x: 3, y: 3 });
        robot.move_robot(1);
        assert_eq!(robot.position, Point { x: 4, y: 4 });
        robot.move_robot(1);
        assert_eq!(robot.position, Point { x: 5, y: 5 });
        robot.move_robot(1);
        assert_eq!(robot.position, Point { x: 6, y: 6 });
        robot.move_robot(1);
        assert_eq!(
            robot.position,
            Point {
                x: 7 % WIDTH,
                y: 7 % HEIGHT
            }
        );
    }

    #[test]
    fn test_safety_factor() {
        let mut restroom = Restroom::new(TESTINPUT);

        assert_eq!(restroom.safety_factor_at_time(100), 12);
    }
}
