type Distance = i64;
type Cost = u64;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ClawMachine {
    ax: Distance,
    ay: Distance,
    bx: Distance,
    by: Distance,
    px: Distance,
    py: Distance,
}

impl ClawMachine {
    pub fn cost(&self) -> Option<Cost> {
        let determinent = self.ax * self.by - self.ay * self.bx;
        if determinent == 0 {
            return None;
        }
        let a_presses = self.px * self.by - self.py * self.bx;
        let b_presses = -self.px * self.ay + self.py * self.ax;
        if a_presses % determinent == 0
            && b_presses % determinent == 0
            && a_presses / determinent >= 0
            && b_presses / determinent >= 0
        {
            Some(((a_presses * 3 + b_presses) / determinent) as Cost)
        } else {
            None
        }
    }
}

pub fn parse_input(input: &str) -> Vec<ClawMachine> {
    fn extract_integers(line: &str) -> (Distance, Distance) {
        let mut v = Vec::new();
        let mut curr = 0;
        for c in line.chars() {
            if let Some(d) = c.to_digit(10) {
                curr = 10 * curr + d;
            } else if curr > 0 {
                v.push(curr as Distance);
                curr = 0;
            }
        }
        if curr > 0 {
            v.push(curr as Distance);
        }
        (v[0], v[1])
    }
    input
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            let (ax, ay) = extract_integers(lines.next().unwrap());
            let (bx, by) = extract_integers(lines.next().unwrap());
            let (px, py) = extract_integers(lines.next().unwrap());
            let px = px + 10000000000000;
            let py = py + 10000000000000;
            ClawMachine {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            }
        })
        .collect()
}

pub fn cost(v: &[ClawMachine]) -> Cost {
    v.iter().filter_map(|c| c.cost()).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cost() {
        let v = super::parse_input(crate::TESTINPUT);
        for c in &v {
            println!("cost: {:?}", c.cost());
        }
        //assert_eq!(super::cost(&v), 480);
        //assert_eq!(super::cost(&v),);
        println!("cost: {}", super::cost(&v));
    }
}
