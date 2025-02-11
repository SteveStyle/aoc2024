/// 'Keypad' includes both the keypad and the controlling robot, or human.
/// The human controlled keypad is keypad zero, regarded as level zero in the heirarchy
/// of keypads.  It controls keypad 1, which controls keypad 2 etc.
///
/// In any step a particular step a keypad is doing one of the following:
///     1)  Pressing a button.  The keypad in the level below is pressing A, as are all
///         lower level keypads.
///     2)  Moving it's finger.  The keypad below is pressing one of the move buttons.  
///         The keypads below that are pressing A.
///     3)  Waiting.  The keypad below is either moving or waiting.  At some level a
///         button is being pressed.
///
/// From the perspective of a particular keypad the whole set of steps can be regarded
/// as a sequence of transitions from one button to another, followed by key presses.
/// Each keypad counts the cost in steps of the buttons it pushes (other than A, which is already
/// counted by the keypads above).  
/// Keypad 0 counts the cost of transitions as free, since the human can just move their fingers.
/// Otherwise a keypad calls a function on the keypad below, which must be a remote, to request a transition and the cost
/// in steps is returned.  A keypad may request different sequences of transitions to find the least
/// cost.
///
/// The function assumes the keypad below starts and ends with it's finger pointing at A, since it
/// is a transition between button presses for the keypad above, or from the starting position.
///
use std::collections::HashMap;

use crate::grid::{Direction, Grid, Point, Vector};

type Cost = isize;

enum KeypadLayoutStyle {
    Room,
    Remote,
}

const ROOM_LAYOUT_TEXT: &str = "789\n456\n123\nX0A";
const REMOTE_LAYOUT_TEXT: &str = "X^A\n<v>";

struct KeypadLayout {
    grid: Grid<u8>,
    map: HashMap<u8, Point>,
    a_key: Point,
    x_key: Point,
}

impl KeypadLayout {
    fn new(text: &str) -> Self {
        let grid = Grid::from(text);
        let mut map = HashMap::new();
        let mut a_key = Point::new(0, 0);
        let mut x_key = Point::new(0, 0);
        for (point, value) in &grid {
            map.insert(*value, point);
            match *value {
                b'A' => a_key = point,
                b'X' => a_key = point,
                _ => {}
            }
        }
        Self {
            grid,
            map,
            a_key,
            x_key,
        }
    }
}

pub struct Keypad {
    layout: KeypadLayout,
    controlled_by: Option<Box<Keypad>>,
    fastest: HashMap<(Point, Point), Cost>,
}

impl Keypad {
    fn new(layout_style: KeypadLayoutStyle, controlled_by: Option<Box<Keypad>>) -> Self {
        let layout = match layout_style {
            KeypadLayoutStyle::Room => KeypadLayout::new(ROOM_LAYOUT_TEXT),
            KeypadLayoutStyle::Remote => KeypadLayout::new(REMOTE_LAYOUT_TEXT),
        };
        Self {
            layout,
            controlled_by,
            fastest: HashMap::new(),
        }
    }
    // The cost of transitioning the finger from 'from' to 'to' on this keypad.
    // Call the transition function on the owned keypad where required.
    fn transition(&mut self, from: Point, to: Point, owner_x_key: Point) -> Cost {
        debug_assert_ne!(from, to);
        if let Some(&cost) = self.fastest.get(&(from, to)) {
            return cost;
        }
        let v = to - from;
        let mut cost = v.x.abs() + v.y.abs();
        let horizontal_key = *self
            .layout
            .map
            .get(&if v.x > 0 { b'>' } else { b'<' })
            .unwrap();
        let vertical_key = *self
            .layout
            .map
            .get(&if v.y > 0 { b'v' } else { b'^' })
            .unwrap();
        if let Some(kp) = &mut self.controlled_by {
            if v.y == 0 {
                cost += kp.transition(self.layout.a_key, horizontal_key, self.layout.x_key)
                    + kp.transition(horizontal_key, self.layout.a_key, self.layout.x_key)
            } else if v.x == 0 {
                cost += kp.transition(self.layout.a_key, vertical_key, self.layout.x_key)
                    + kp.transition(vertical_key, self.layout.a_key, self.layout.x_key)
            } else {
                let mut transition_cost = isize::MAX;
                let horizontal_range = if from.x <= to.x {
                    (from.x..=to.x)
                } else {
                    (to.x..=from.x)
                };
                let vertical_range = if from.y <= to.y {
                    (from.y..=to.y)
                } else {
                    (to.y..=from.y)
                };
                if !((owner_x_key.y == from.y && horizontal_range.contains(&owner_x_key.x))
                    || (owner_x_key.x == to.x && vertical_range.contains(&owner_x_key.y)))
                {
                    transition_cost =
                        kp.transition(self.layout.a_key, horizontal_key, self.layout.x_key)
                            + kp.transition(horizontal_key, vertical_key, self.layout.x_key)
                            + kp.transition(vertical_key, self.layout.a_key, self.layout.x_key);
                }
                if !((owner_x_key.x == from.x && vertical_range.contains(&owner_x_key.y))
                    || (owner_x_key.y == to.y && horizontal_range.contains(&owner_x_key.x)))
                {
                    transition_cost = transition_cost.min(
                        kp.transition(self.layout.a_key, vertical_key, self.layout.x_key)
                            + kp.transition(vertical_key, horizontal_key, self.layout.x_key)
                            + kp.transition(horizontal_key, self.layout.a_key, self.layout.x_key),
                    );
                }
            }
        }

        self.fastest.insert((from, to), cost);
        cost
    }

    fn cost_for_sequence(&mut self, sequence: &[u8]) -> Cost {
        let mut cost = 0;
        let mut curr_key = self.layout.a_key;

        if let Some(kp) = &mut self.controlled_by {
            for &next_key in sequence {
                if let Some(&next_key) = self.layout.map.get(&next_key) {
                    cost += kp.transition(curr_key, next_key, self.layout.x_key);

                    cost += 1;
                    curr_key = next_key;
                } else {
                    unreachable!("next key {}", next_key);
                }
            }
        }

        cost
    }
    pub fn cost_for_targets(&mut self, targets: &Vec<[u8; 4]>) -> Cost {
        let mut cost = 0;
        for sequence in targets {
            let digits: Cost = parse(&sequence[0..sequence.len()]);
            cost += self.cost_for_sequence(sequence) * digits;
        }
        cost
    }
}

fn parse(digits: &[u8]) -> Cost {
    let mut result = 0;
    for digit in digits {
        result = result * 10 + *digit as Cost;
    }
    result
}

pub struct Scenario {
    room_keypad: Keypad,
    targets: Vec<[u8; 4]>,
}

impl Scenario {
    pub fn new(input: &str) -> Self {
        let mut targets = Vec::with_capacity(4);
        for line in input.lines() {
            let mut bytes = line.bytes();
            targets.push([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ]);
        }
        let human_keypad = Keypad::new(KeypadLayoutStyle::Remote, None);
        let remote_keypad1 = Keypad::new(KeypadLayoutStyle::Remote, Some(Box::new(human_keypad)));
        let remote_keypad2 = Keypad::new(KeypadLayoutStyle::Remote, Some(Box::new(remote_keypad1)));
        let room_keypad = Keypad::new(KeypadLayoutStyle::Room, Some(Box::new(remote_keypad2)));
        Self {
            room_keypad,
            targets,
        }
    }

    pub fn cost_for_targets(&mut self) -> Cost {
        self.room_keypad.cost_for_targets(&self.targets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_cost_for_sequence() {
        let mut scenario = Scenario::new(TESTINPUT);
        let cost = scenario.cost_for_targets();
        assert_eq!(cost, 126384);
    }
}
