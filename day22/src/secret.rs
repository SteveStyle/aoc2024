use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

type SecretNumber = i64;

const PRUNE: SecretNumber = 16777216;

#[derive(Debug, Copy, Clone)]
pub struct Secret(SecretNumber);

impl Deref for Secret {
    type Target = SecretNumber;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Secret {
    fn new(sn: SecretNumber) -> Self {
        Self(sn)
    }
    fn mix(&mut self, other: SecretNumber) {
        self.0 ^= other;
    }
    fn prune(&mut self) {
        self.0 %= PRUNE;
    }
    fn next(&mut self) {
        self.mix(self.0 << 6);
        self.prune();

        self.mix(self.0 >> 5);
        self.prune();

        self.mix(self.0 << 11);
        self.prune();
    }
    fn next_n(&mut self, n: SecretNumber) -> SecretNumber {
        for _ in 0..n {
            self.next();
        }
        self.0
    }
    fn price(&self) -> i8 {
        (self.0 % 10) as i8
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Queue4([i8; 4]);

impl Queue4 {
    fn new() -> Self {
        Self([0; 4])
    }
    fn push(&mut self, new_value: i8) {
        self.0[0] = self.0[1];
        self.0[1] = self.0[2];
        self.0[2] = self.0[3];
        self.0[3] = new_value;
    }
    fn as_slice(&self) -> &[i8; 4] {
        &self.0
    }
}

impl Deref for Queue4 {
    type Target = [i8; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Queue2([i8; 2]);

impl Queue2 {
    fn new() -> Self {
        Self([0; 2])
    }
    fn push(&mut self, new_value: i8) {
        self.0[0] = self.0[1];
        self.0[1] = new_value;
    }
    fn as_slice(&self) -> &[i8; 2] {
        &self.0
    }
    fn delta(&self) -> i8 {
        self.0[1] - self.0[0]
    }
}

impl Deref for Queue2 {
    type Target = [i8; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn parse_input(input: &str) -> Vec<Secret> {
    let mut result = Vec::with_capacity(1635);
    for line in input.lines() {
        result.push(Secret::new(line.parse().unwrap()));
    }
    result
}

pub fn sum_secrets(secrets: &mut [Secret]) -> SecretNumber {
    let mut total = 0;
    for secret in secrets {
        total += secret.next_n(2000);
    }
    total
}

fn update_totals(totals: &mut HashMap<[i8; 4], SecretNumber>, mut secret: Secret) {
    let mut found: HashSet<[i8; 4]> = HashSet::new();
    let mut prices = Queue2::new();
    let mut deltas = Queue4::new();

    prices.push(secret.price());
    for _ in 0..3 {
        secret.next();
        prices.push(secret.price());
        deltas.push(prices.delta());
    }
    for _ in 0..(2000 - 3) {
        secret.next();
        prices.push(secret.price());
        deltas.push(prices.delta());
        if !found.contains(deltas.as_slice()) {
            let entry = totals.entry(*deltas.as_slice()).or_insert(0);
            *entry += prices.0[1] as SecretNumber;
            found.insert(*deltas.as_slice());
        }
    }
}

pub fn most_bananas(secrets: &mut [Secret]) -> SecretNumber {
    let mut totals: HashMap<[i8; 4], SecretNumber> = HashMap::new();
    for secret in secrets {
        update_totals(&mut totals, *secret);
    }

    *totals.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_examples() {
        assert_eq!(Secret::new(1).next_n(2000), 8685429);
    }

    #[test]
    fn test_all_examples() {
        let mut secrets = parse_input(TESTINPUT);
        assert_eq!(sum_secrets(&mut secrets), 37327623);
    }

    #[test]
    fn test_most_bananas() {
        let mut secrets = parse_input(TESTINPUT2);
        let most_bananas = most_bananas(&mut secrets);
        assert_eq!(most_bananas, 23);
    }
}
