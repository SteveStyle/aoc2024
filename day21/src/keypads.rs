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
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, AddAssign},
};

use crate::grid::{Grid, Point, Vector};

type Cost = isize;

enum KeypadLayoutStyle {
    Room,
    Remote,
}

const ROOM_LAYOUT_TEXT: &str = "789\n456\n123\nX0A";
const REMOTE_LAYOUT_TEXT: &str = "X^A\n<v>";
#[derive(Debug, Clone)]
struct KeypadLayout {
    // grid: Grid<u8>,
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
                b'X' => x_key = point,
                _ => {}
            }
        }
        Self {
            // grid,
            map,
            a_key,
            x_key,
        }
    }
}
#[derive(Clone, PartialEq)]
struct Sequence {
    cost: Cost,
    // moves: Vec<u8>,
}

impl Sequence {
    // fn new(cost: Cost, moves: Vec<u8>) -> Self {
    fn new(cost: Cost) -> Self {
        Self { cost }
        // Self { cost, moves }
    }
    fn min(self, other: Self) -> Self {
        if self.cost <= other.cost {
            self
        } else {
            other
        }
    }
}

const EMPTY_SEQUENCE: Sequence = Sequence {
    cost: 0,
    // moves: Vec::new(),
};

impl Add for Sequence {
    type Output = Sequence;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
            // moves: [self.moves, rhs.moves].concat(),
        }
    }
}
impl AddAssign for Sequence {
    // fn add_assign(&mut self, mut rhs: Self) {
    fn add_assign(&mut self, rhs: Self) {
        self.cost += rhs.cost;
        // self.moves.append(&mut rhs.moves);
    }
}
impl AddAssign<&mut Sequence> for Sequence {
    fn add_assign(&mut self, rhs: &mut Self) {
        self.cost += rhs.cost;
        // self.moves.append(&mut rhs.moves);
    }
}

impl Debug for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let moves: String = self.moves.iter().map(|&b| b as char).collect();
        f.debug_struct("Sequence")
            .field("cost", &self.cost)
            // .field("moves", &moves)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Keypad {
    layout: KeypadLayout,
    controlled_by: Option<Box<Keypad>>,
    fastest: HashMap<(Point, Point), Sequence>,
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
    #[allow(unused_variables)]
    fn push_button(&self, button: Point, repetitions: Cost) -> Sequence {
        // If there is a remote control then it should push A so that we push the button,
        //  otherwise we are the human and push the actual button directly.
        // let button = self.layout.grid[button];
        if self.controlled_by.is_none() {
            // Sequence::new(repetitions, vec![button; repetitions as usize])
            Sequence::new(repetitions)
        } else {
            // Sequence::new(repetitions, vec![b'A'; repetitions as usize])
            Sequence::new(repetitions)
        }
    }

    fn move_this_finger(&mut self, from: Point, to: Point) -> Sequence {
        if let Some(kp) = &mut self.controlled_by {
            kp.move_owner_finger(from, to, self.layout.x_key)
        } else {
            EMPTY_SEQUENCE
        }
    }
    // The cost of transitioning the finger from 'from' to 'to' on this keypad.
    // Call the transition function on the owned keypad where required.
    fn move_owner_finger(&mut self, from: Point, to: Point, owner_x_key: Point) -> Sequence {
        debug_assert_ne!(from, to);
        if let Some(sequence) = self.fastest.get(&(from, to)) {
            return sequence.clone();
        }
        let mut v = to - from;
        let mut sequence = EMPTY_SEQUENCE;
        // let mut sequence = v.x.abs() + v.y.abs();

        let a_key = self.layout.a_key;
        let horizontal_key = self.horizontal_key(v);
        let vertical_key = self.vertical_key(v);
        v = v.abs();

        if v.y == 0 {
            sequence += self.move_this_finger(self.layout.a_key, horizontal_key);
            sequence += self.push_button(horizontal_key, v.x);
            sequence += self.move_this_finger(horizontal_key, self.layout.a_key);
        } else if v.x == 0 {
            sequence += self.move_this_finger(self.layout.a_key, vertical_key);
            sequence += self.push_button(vertical_key, v.y);
            sequence += self.move_this_finger(vertical_key, self.layout.a_key);
        } else {
            let horizontal_range = horizontal_range(from, to);
            let vertical_range = vertical_range(from, to);
            let horizontal_first = if !((owner_x_key.y == from.y
                && horizontal_range.contains(&owner_x_key.x))
                || (owner_x_key.x == to.x && vertical_range.contains(&owner_x_key.y)))
            {
                let mut horizontal_first = EMPTY_SEQUENCE;
                horizontal_first += self.move_this_finger(a_key, horizontal_key);
                horizontal_first += self.push_button(horizontal_key, v.x);
                horizontal_first += self.move_this_finger(horizontal_key, vertical_key);
                horizontal_first += self.push_button(vertical_key, v.y);
                horizontal_first += self.move_this_finger(vertical_key, a_key);
                Some(horizontal_first)
            } else {
                None
            };
            let vertical_first = if !((owner_x_key.x == from.x
                && vertical_range.contains(&owner_x_key.y))
                || (owner_x_key.y == to.y && horizontal_range.contains(&owner_x_key.x)))
            {
                let mut vertical_first = EMPTY_SEQUENCE;
                vertical_first += self.move_this_finger(a_key, vertical_key);
                vertical_first += self.push_button(vertical_key, v.y);
                vertical_first += self.move_this_finger(vertical_key, horizontal_key);
                vertical_first += self.push_button(horizontal_key, v.x);
                vertical_first += self.move_this_finger(horizontal_key, a_key);
                Some(vertical_first)
            } else {
                None
            };
            sequence += match (horizontal_first, vertical_first) {
                (None, None) => unreachable!(),
                (None, Some(vs)) => vs,
                (Some(hs), None) => hs,
                (Some(hs), Some(vs)) => hs.min(vs),
            }
        }

        self.fastest.insert((from, to), sequence.clone());
        sequence
    }

    fn vertical_key(&mut self, v: Vector) -> Point {
        let vertical_key = *self
            .layout
            .map
            .get(&if v.y > 0 { b'v' } else { b'^' })
            .unwrap();
        vertical_key
    }

    fn horizontal_key(&mut self, v: Vector) -> Point {
        let horizontal_key = *self
            .layout
            .map
            .get(&if v.x > 0 { b'>' } else { b'<' })
            .unwrap();
        horizontal_key
    }

    fn enter_code(&mut self, code: &[u8]) -> Sequence {
        let mut sequence = EMPTY_SEQUENCE;
        let mut curr_key = self.layout.a_key;
        for &next_key in code {
            if let Some(&next_key) = self.layout.map.get(&next_key) {
                sequence += self.move_this_finger(curr_key, next_key);
                sequence += self.push_button(next_key, 1);
                curr_key = next_key;
            } else {
                unreachable!("next key {}", next_key);
            }
        }

        sequence
    }
    pub fn cost_for_targets(&mut self, targets: &Vec<[u8; 4]>) -> Cost {
        let mut cost = 0;
        for code in targets {
            let digits: Cost = parse(&code[0..code.len() - 1]);
            cost += self.enter_code(code).cost * digits;
        }
        cost
    }
}

fn vertical_range(from: Point, to: Point) -> std::ops::RangeInclusive<usize> {
    if from.y <= to.y {
        from.y..=to.y
    } else {
        to.y..=from.y
    }
}

fn horizontal_range(from: Point, to: Point) -> std::ops::RangeInclusive<usize> {
    if from.x <= to.x {
        from.x..=to.x
    } else {
        to.x..=from.x
    }
}

fn parse(digits: &[u8]) -> Cost {
    let mut result = 0;
    for &digit in digits {
        result = result * 10 + (digit - b'0') as Cost;
    }
    result
}

#[derive(Debug, Clone)]
pub struct Scenario {
    room_keypad: Keypad,
    targets: Vec<[u8; 4]>,
}

impl Scenario {
    pub fn new(input: &str, no_of_remotes: Cost) -> Self {
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

        let mut controlled_by = None;
        for _ in 0..no_of_remotes {
            let new_keypad = Keypad::new(KeypadLayoutStyle::Remote, controlled_by);
            controlled_by = Some(Box::new(new_keypad));
        }
        let room_keypad = Keypad::new(KeypadLayoutStyle::Room, controlled_by);
        Self {
            room_keypad,
            targets,
        }
    }

    pub fn cost_for_targets(&mut self) -> Cost {
        self.room_keypad.cost_for_targets(&self.targets)
    }

    #[cfg(test)]
    fn enter_code(&mut self, code: &[u8]) -> Sequence {
        self.room_keypad.enter_code(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_cost_for_sequence() {
        let mut scenario = Scenario::new(TESTINPUT, 3);
        let cost = scenario.cost_for_targets();
        assert_eq!(cost, 126384);
    }

    #[test]
    fn test_029a() {
        let mut scenario = Scenario::new(TESTINPUT, 3);
        let code: Vec<u8> = "029A".bytes().collect();
        let sequence = scenario.enter_code(&code);
        println!("{:?}", sequence);
    }
    #[test]
    fn test_cost_for_sequence2() {
        let mut scenario = Scenario::new(TESTINPUT, 26);
        let cost = scenario.cost_for_targets();
        println!("total cost {cost}")
        // assert_eq!(cost, 126384);
    }

    #[test]
    fn test_029a_2() {
        let mut scenario = Scenario::new(TESTINPUT, 26);
        let code: Vec<u8> = "029A".bytes().collect();
        let sequence = scenario.enter_code(&code);
        println!("{:?}", sequence);
    }
}
