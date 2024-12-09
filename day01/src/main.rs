use std::time::Instant;

mod calcdistance;

mod timer;

// import input.txt as a static string
const INPUT: &str = include_str!("input.txt");

fn main() {
    let now = Instant::now();
    let t_parse_input = timer::time(|| calcdistance::parse_input(INPUT), "parse_input");

    let (a, b) = &*t_parse_input;

    let distance = {
        let a = a.clone();
        let b = b.clone();
        timer::time(|| calcdistance::calc_distance(a, b), "calc_distance")
    };

    let repeats1 = {
        let a = a.clone();
        let b = b.clone();
        timer::time(|| calcdistance::count_repeats1(a, b), "count_repeats1")
    };

    let repeats2 = {
        let a = a.clone();
        let b = b.clone();
        timer::time(|| calcdistance::count_repeats2(a, b), "count_repeats2")
    };

    let repeats3 = {
        let a = a.clone();
        let b = b.clone();
        timer::time(|| calcdistance::count_repeats3(a, b), "count_repeats3")
    };

    let main_duration = now.elapsed();

    t_parse_input.print_duration();
    distance.print_all();
    repeats1.print_all();
    repeats2.print_all();
    repeats3.print_all();

    println!("Main duration: {:?}", main_duration);
}
