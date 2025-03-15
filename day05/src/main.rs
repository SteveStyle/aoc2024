use stephen_morris_utils::timer;

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
const TESTINPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

mod print_rules;

fn main() {
    let ru = timer::time(|| print_rules::parse_input(INPUT), "parse_input");

    let sum = timer::time(|| print_rules::test_updates(&ru.0, &ru.1), "test_updates");

    let reorder_sum = timer::time(|| print_rules::reorder(&ru.0, &ru.1), "reorder");

    ru.print_duration();
    sum.print_all();
    reorder_sum.print_all();
}
