#![allow(dead_code)]
#![feature(slice_group_by)]
mod day1;
mod day2;

fn main() {
    let input = std::fs::read_to_string("../input/day2.txt").unwrap();
    println!("{:?}", day2::part_1(input.clone()));
    println!("{:?}", day2::part_2(input));
}
