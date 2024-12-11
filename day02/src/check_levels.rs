pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn check_report(report: &mut [i32]) -> bool {
    if report[0] > report[1] {
        report.reverse();
    }
    for i in 0..report.len() - 1 {
        if !(1..=3).contains(&(report[i + 1] - report[i])) {
            return false;
        };
    }
    true
}

pub fn check_reports(reports: &mut [Vec<i32>]) -> i32 {
    let mut count = 0;
    for report in reports {
        count += check_report(report) as i32;
    }
    count
}

#[allow(unused_imports)]
mod tests {
    use crate::TESTINPUT;

    use super::*;

    #[test]
    fn test_parse_input() {
        let mut reports = parse_input(TESTINPUT);
        assert_eq!(reports.len(), 6);
        let count = check_reports(&mut reports);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_bool_conversion() {
        assert_eq!(true as i32, 1);
        assert_eq!(false as i32, 0);
    }
}
