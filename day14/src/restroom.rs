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

use crate::grid::{Point, Vector};
use stephen_morris_utils::get_numbers;
use stephen_morris_utils::pos;

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
    pub fn move_robot(&mut self, seconds: isize) -> &Self {
        let mut position = Vector::from(self.position);
        position += self.velocity * seconds;
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

pub fn safety_factor(robots: &mut [Robot]) -> usize {
    let mut counts = [0; 4];
    robots
        .iter_mut()
        .filter_map(|robot| robot.move_robot(100).quadrant())
        .for_each(|quadrant| counts[quadrant - 1] += 1);
    counts.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut robots = parse_input(TESTINPUT);

        assert_eq!(safety_factor(&mut robots), 12);
    }
}
