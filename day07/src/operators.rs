use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone)]
pub struct Equation {
    target: usize,
    parameters: Vec<usize>,
}

impl Equation {
    pub fn new(target: usize, parameters: Vec<usize>) -> Self {
        Equation { target, parameters }
    }
}

pub enum EquationParseError {
    NoParams,
    ParseIntError(ParseIntError),
}

impl TryFrom<&str> for Equation {
    type Error = EquationParseError;
    fn try_from(line: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        let mut split = line.split_whitespace();
        let target = split.next().ok_or(EquationParseError::NoParams)?;
        let target = target[..target.len() - 1]
            .parse()
            .map_err(EquationParseError::ParseIntError)?;
        let mut parameters = Vec::new();
        for number in split {
            parameters.push(number.parse().map_err(EquationParseError::ParseIntError)?)
        }
        if parameters.is_empty() {
            return Err(EquationParseError::NoParams);
        }
        Ok(Equation { target, parameters })
    }
}

pub fn parse_input(input: &str) -> Vec<Equation> {
    let ret = input
        .lines()
        .filter_map(|s| Equation::try_from(s).ok())
        .collect();

    ret
}

fn solve(target: usize, first: usize, rest: &[usize]) -> Option<usize> {
    if first > target {
        return None;
    }
    if rest.is_empty() {
        return if first == target { Some(target) } else { None };
    }
    if solve(target, first + rest[0], &rest[1..]).is_some()
        || solve(target, first * rest[0], &rest[1..]).is_some()
    {
        return Some(target);
    }
    None
}

fn solve2(target: usize, first: usize, rest: &[usize]) -> Option<usize> {
    fn concat(a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b_copy = b;
        while b_copy > 0 {
            a *= 10;
            b_copy /= 10;
        }
        a + b
    }
    if first > target {
        return None;
    }
    if rest.is_empty() {
        return if first == target { Some(target) } else { None };
    }
    if solve2(target, first + rest[0], &rest[1..]).is_some()
        || solve2(target, first * rest[0], &rest[1..]).is_some()
        || solve2(target, concat(first, rest[0]), &rest[1..]).is_some()
    {
        return Some(target);
    }
    None
}

impl Equation {
    pub fn solve(&self) -> Option<usize> {
        solve(self.target, self.parameters[0], &self.parameters[1..])
    }
    pub fn solve2(&self) -> Option<usize> {
        solve2(self.target, self.parameters[0], &self.parameters[1..])
    }
}

pub fn solve_and_sum(v: &[Equation]) -> usize {
    v.iter().filter_map(|e| e.solve()).sum()
}

pub fn solve_and_sum2(v: &[Equation]) -> usize {
    v.iter().filter_map(|e| e.solve2()).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_input() {
        let equations = super::parse_input(crate::TESTINPUT);
        println!("{equations:#?}");
    }

    #[test]
    fn test_solve() {
        use super::solve;
        assert_eq!(solve(10, 10, &[]), Some(10));
        assert_eq!(solve(10, 15, &[]), None);
        assert_eq!(solve(10, 5, &[]), None);
        assert_eq!(solve(10, 5, &[5]), Some(10));
        assert_eq!(solve(10, 5, &[2]), Some(10));
        assert_eq!(solve(10, 5, &[1, 4]), Some(10));
        assert_eq!(solve(10, 0, &[2, 3, 4]), Some(10));
        assert_eq!(solve(10, 0, &[2, 3, 3]), None);
    }

    #[test]
    fn test_solve_and_sum() {
        let equations = super::parse_input(crate::TESTINPUT);
        assert_eq!(super::solve_and_sum(&equations), 3749);
    }

    #[test]
    fn test_solve_and_sum2() {
        let equations = super::parse_input(crate::TESTINPUT);
        for e in &equations {
            println!("{:?} {:?}", e, e.solve2());
        }
        assert_eq!(super::solve_and_sum2(&equations), 11387);
    }

    #[test]
    fn test_concat() {
        fn concat(a: usize, b: usize) -> usize {
            let mut a = a;
            let mut b_copy = b;
            while b_copy > 0 {
                a *= 10;
                b_copy /= 10;
            }
            a + b
        }
        println!("{}", concat(123, 456));
    }
}
