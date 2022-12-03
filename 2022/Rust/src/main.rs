#![allow(dead_code)]
#![feature(slice_group_by)]
#![feature(iter_array_chunks)]
mod day1;
mod day2;
mod day3;

fn main() {
    let input = std::fs::read_to_string("../input/day3.txt").unwrap();
    println!("{:?}", day3::part_1(&input));
    println!("{:?}", day3::part_2(&input));
}
