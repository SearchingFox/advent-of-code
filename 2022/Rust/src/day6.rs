use std::{
    collections::HashSet,
    ops::ControlFlow::{Break, Continue},
};

pub fn part_1(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .try_fold(0, |acc, w| {
            if HashSet::<_>::from_iter(w).len() != 4 {
                Continue(acc+1)
            } else {
                Break(acc)
            }
        }).break_value().unwrap()
        // .take_while(|&w| HashSet::<&char>::from_iter(w).len() != 4)
        // .count()
        + 4
}

pub fn part_2(input: &str) -> usize {
    for (c, w) in input.chars().collect::<Vec<_>>().windows(14).enumerate() {
        if w.iter().collect::<HashSet<_>>().len() == 14 {
            return c + 14;
        }
    }
    0
}
