#![allow(dead_code, unused)]
use stephen_morris_utils::timer;

use std::num::NonZeroU8;

const INPUT: &str = include_str!("input.txt");
#[allow(dead_code)]
const TESTINPUT: &str = "2333133121414131402";

mod compact;
mod compact2;
mod vector_linked_list;

fn main() {
    let disk = timer::time(|| compact::parse_input(INPUT), "parse_input");
    let compacted_disk = timer::time(|| compact::compact_disk(&disk), "compact_disk");
    let checksum = timer::time(|| compact::checksum(&compacted_disk), "checksum");

    disk.print_duration();
    compacted_disk.print_duration();
    checksum.print_all();

    let file_map = timer::time(|| compact2::parse_input(INPUT), "parse_input");
    let compact_disk = timer::time(|| compact2::compact_disk(&file_map), "compact_disk");
    let checksum = timer::time(|| compact2::checksum(&compact_disk), "checksum");

    file_map.print_duration();
    compact_disk.print_duration();
    checksum.print_all();
}
