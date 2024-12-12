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

pub fn check_report_asc(report: &[i32]) -> bool {
    for i in 0..report.len() - 1 {
        if !(1..=3).contains(&(report[i + 1] - report[i])) {
            return false;
        };
    }
    true
}

pub fn check_report_desc(report: &[i32]) -> bool {
    for i in 0..report.len() - 1 {
        if !(-3..=-1).contains(&(report[i + 1] - report[i])) {
            return false;
        };
    }
    true
}

pub fn check_report(report: &[i32]) -> bool {
    if check_report_asc(report) {
        return true;
    }
    if check_report_desc(report) {
        return true;
    }
    false
}

pub fn check_reports(reports: &mut [Vec<i32>]) -> i32 {
    let mut count = 0;
    for report in reports {
        count += check_report(report) as i32;
    }
    count
}

pub fn check_report2(report: &mut [i32]) -> bool {
    if check_report(report) {
        return true;
    }
    if check_report(&report[1..]) {
        return true;
    }
    if check_report(&report[..report.len() - 1]) {
        return true;
    }
    if report[0] > report[report.len() - 1] {
        report.reverse();
    }
    let mut issue_found = false;
    let mut i = 0;
    while i < report.len() - 1 {
        if !(1..=3).contains(&(report[i + 1] - report[i])) {
            // if we have already missed out a value then we can't miss out another one, so fail
            if issue_found {
                return false;
            }
            // i+2 must be in range, or we would have already failed.  i-1 might not be in range if i=0.
            // Our only chance is to miss out i or i+1 so we need either i-1, i+1, i+2 or i-1, i, i+2 to make a valid sequence.
            if (1..=3).contains(&(report[i + 2] - report[i]))
                || (i > 0)
                    && (1..=3).contains(&(report[i + 1] - report[i - 1]))
                    && (1..=3).contains(&(report[i + 2] - report[i + 1]))
            {
                issue_found = true;
                i += 1;
            // if none of the above conditions are met then we can't miss out a value, so fail
            } else {
                return false;
            }
        }
        i += 1;
    }
    true
}

pub fn check_reports2(reports: &mut [Vec<i32>]) -> i32 {
    let mut count = 0;
    for report in reports {
        count += check_report2(report) as i32;
    }
    count
}

#[allow(unused_imports, dead_code)]

mod tests {
    use crate::TESTINPUT;

    use super::*;

    fn check_report_test(report: &[i32]) -> bool {
        //generate vectors missing out one value at a time
        for i in 0..report.len() {
            let mut report2 = report.to_vec();
            report2.remove(i);
            if check_report(report2.as_slice()) {
                return true;
            }
        }
        false
    }

    fn check_reports2_test(reports: &mut [Vec<i32>]) {
        let mut count = 0;
        for report in reports {
            if check_report2(report) != check_report_test(report) {
                println!(
                    "Failed for {:?}, check_report2 {:?}, check_report_test {:?}",
                    report,
                    check_report2(&mut report.clone()),
                    check_report_test(&report.clone())
                );
                count += 1;
            }
        }
        println!("Failed {} tests", count);
    }

    #[test]
    fn check_report_test1() {
        let reports = parse_input(crate::INPUT);
        check_reports2_test(&mut reports.clone());
    }

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

    #[test]
    fn test_check_reports2() {
        let mut reports = parse_input(TESTINPUT);
        assert_eq!(reports.len(), 6);
        let count = check_reports2(&mut reports);
        assert_eq!(count, 4);
    }
}
