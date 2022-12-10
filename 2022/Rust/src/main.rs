#![allow(dead_code)]
#![feature(slice_group_by)]
#![feature(iter_array_chunks)]
#![feature(control_flow_enum)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() {
    let input = std::fs::read_to_string("../input/day8.txt").unwrap();
    println!("Part 1: {:?}", day8::part_1(&input));
    println!("Part 2: {:?}", day8::part_2(&input));
}
