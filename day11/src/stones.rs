use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn g(n: u64) -> Vec<u64> {
    if n == 0 {
        vec![1]
    } else {
        let s = n.to_string();
        let l = s.len();
        if l % 2 == 0 {
            vec![s[0..l / 2].parse().unwrap(), (s[l / 2..]).parse().unwrap()]
        } else {
            vec![n * 2024]
        }
    }
}

pub fn f(a: u8, n: u64, fhash: &mut HashMap<(u8, u64), usize>) -> usize {
    match fhash.get(&(a, n)) {
        Some(&f) => f,
        None => {
            let f = f_sub(a, n, fhash);
            fhash.insert((a, n), f);
            f
        }
    }

    //    *fhash.entry((a, n)).or_insert_with(|| f_sub(a, n))
}

fn f_sub(a: u8, n: u64, fhash: &mut HashMap<(u8, u64), usize>) -> usize {
    if a == 0 {
        1
    } else {
        g(n).iter().map(|&n| f(a - 1, n, fhash)).sum()
    }
}

pub fn f_list(a: u8, v: &[u64], fhash: &mut HashMap<(u8, u64), usize>) -> usize {
    v.iter().map(|&n| f(a, n, fhash)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut fhash = HashMap::new();
        let f = f(7, 0, &mut fhash);
        println!("{f}")
    }

    #[test]
    fn test_test_input() {
        let mut fhash = HashMap::new();
        let v = parse_input(crate::TESTINPUT);
        let f = f_list(6, &v, &mut fhash);
        assert_eq!(f, 22);
        let f = f_list(25, &v, &mut fhash);
        assert_eq!(f, 55312);
    }
    #[test]
    fn test_g() {
        assert_eq!(g(0), vec![1]);
        assert_eq!(g(1), vec![2024]);
        assert_eq!(g(4), vec![4 * 2024]);
        assert_eq!(g(4979), vec![49, 79]);
        assert_eq!(g(24), vec![2, 4]);
        assert_eq!(g(4356119), vec![4356119 * 2024]);
        assert_eq!(g(914), vec![914 * 2024]);
        assert_eq!(g(857345), vec![857, 345]);
        assert_eq!(g(698829), vec![698, 829]);
        assert_eq!(g(698001), vec![698, 1]);
    }
}
