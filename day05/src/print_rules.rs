use std::{collections::HashMap, ops::Deref, ops::DerefMut};

type Rules = Vec<(usize, usize)>;

#[derive(Debug, Clone)]
pub struct Update(Vec<usize>);

impl Deref for Update {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Update {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn parse_input(input: &str) -> (Rules, Vec<Update>) {
    let no_lines = input.lines().count();
    let mut rules = Vec::with_capacity(no_lines >> 1);
    let mut update_list = Vec::with_capacity(no_lines >> 1);
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        };
        if line.contains('|') {
            let mut parts = line.split('|');
            rules.push((
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ));
        } else {
            update_list.push(Update(
                line.split(',').map(|s| s.parse().unwrap()).collect(),
            ))
        }
    }

    (rules, update_list)
}

fn test_update(rules: &Rules, update: &Update) -> bool {
    let mut index: [Option<usize>; 100] = [None; 100];
    for (i, value) in update.0.iter().enumerate() {
        index[*value] = Some(i);
    }
    for (a, b) in rules {
        if let (Some(a_index), Some(b_index)) = (index[*a], index[*b]) {
            if b_index < a_index {
                return false;
            };
        }
    }

    true
}

pub fn test_updates(rules: &Rules, updates: &[Update]) -> usize {
    updates
        .iter()
        .filter(|u| test_update(rules, u))
        .map(|u| u[u.len() >> 1])
        .sum()
}

pub fn reorder(rules: &Rules, updates: &[Update]) -> usize {
    let mut hm_rules: HashMap<(usize, usize), std::cmp::Ordering> =
        HashMap::with_capacity((rules.len() * (rules.len() - 1)) >> 1);
    for (a, b) in rules {
        hm_rules.insert((*a, *b), std::cmp::Ordering::Less);
        hm_rules.insert((*b, *a), std::cmp::Ordering::Greater);
    }
    updates
        .iter()
        .filter(|u| !test_update(rules, u))
        .map(|u| {
            let mut u = u.clone();
            u.sort_by(|a, b| *hm_rules.get(&(*a, *b)).unwrap());
            u
        })
        .map(|u| u[u.len() >> 1])
        .sum()
}
#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::test_updates;

    #[test]
    fn test_test_updates() {
        let (rules, updates) = parse_input(crate::TESTINPUT);
        assert_eq!(test_updates(&rules, &updates), 143);
    }

    #[test]
    fn test_reorder() {
        let (rules, updates) = parse_input(crate::TESTINPUT);
        assert_eq!(super::reorder(&rules, &updates), 123);
    }
}
