use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .map(|w| (HashSet::<&char>::from_iter(w).len() == 4).then_some(()))
        .take_while(Option::is_none)
        .count()
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
