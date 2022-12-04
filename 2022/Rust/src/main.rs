#![allow(dead_code)]
#![feature(slice_group_by)]
#![feature(iter_array_chunks)]
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let input = std::fs::read_to_string("../input/day4.txt").unwrap();
    println!("{:?}", day4::part_1(&input));
    println!("{:?}", day4::part_2(&input));
}
