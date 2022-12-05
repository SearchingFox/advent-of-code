#![allow(dead_code)]
#![feature(slice_group_by)]
#![feature(iter_array_chunks)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input = std::fs::read_to_string("../input/day5.txt").unwrap();
    println!("{:?}", day5::part_1(&input));
    println!("{:?}", day5::part_2(&input));
}
