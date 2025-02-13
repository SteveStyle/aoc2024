type SecretNumber = u64;

const PRUNE: SecretNumber = 16777216;

#[derive(Debug)]
pub struct Secret(SecretNumber);
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
}
