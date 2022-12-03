use std::collections::HashSet;

fn char_to_u32(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| {
            let (t1, t2) = line.split_at(line.len() / 2);
            t1.chars()
                .collect::<HashSet<_>>()
                .intersection(&t2.chars().collect::<HashSet<_>>())
                .map(|&x| char_to_u32(x))
                .collect::<Vec<u32>>()
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks()
        .flat_map(|[x, y, z]| {
            let sets = [
                x.chars().map(|c| char_to_u32(c)).collect::<HashSet<u32>>(),
                y.chars().map(|c| char_to_u32(c)).collect::<HashSet<u32>>(),
                z.chars().map(|c| char_to_u32(c)).collect::<HashSet<u32>>(),
            ];

            sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
                acc.intersection(&hs).cloned().collect()
            })
        })
        .sum()
}
