use std::time::Instant;

mod calcdistance;

mod timer;

// import input.txt as a static string
const INPUT: &str = include_str!("input.txt");

fn main() {
    let now = Instant::now();

    let ((a, b), parse_input_duration) = calcdistance::parse_input(INPUT);
    let (distance, calc_distance_duration) = calcdistance::calc_distance(a.clone(), b.clone());

    let (repeats1, count_repeats1_duration) = calcdistance::count_repeats1(a.clone(), b.clone());

    let (repeats2, count_repeats2_duration) = calcdistance::count_repeats2(a.clone(), b.clone());

    let (repeats3, count_repeats3_duration) = calcdistance::count_repeats3(a.clone(), b.clone());

    let main_duration = now.elapsed();

    println!("Distance: {}", distance);
    println!("Repeats1: {}", repeats1);
    println!("Repeats2: {}", repeats2);
    println!("Repeats3: {}", repeats3);
    println!("Parse input duration: {:?}", parse_input_duration);
    println!("Calc distance duration: {:?}", calc_distance_duration);
    println!("Count repeats1 duration: {:?}", count_repeats1_duration);
    println!("Count repeats2 duration: {:?}", count_repeats2_duration);
    println!("Count repeats3 duration: {:?}", count_repeats3_duration);
    println!("Main duration: {:?}", main_duration);
}
