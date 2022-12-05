use std::collections::BTreeSet;

fn str_to_u32((a, b): (&str, &str)) -> (u32, u32) {
    (a.parse().unwrap(), b.parse().unwrap())
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (p1, p2) = line.split_once(',').unwrap();
            let (p1_start, p1_end) = str_to_u32(p1.split_once('-').unwrap());
            let (p2_start, p2_end) = str_to_u32(p2.split_once('-').unwrap());

            p1_start >= p2_start && p1_end <= p2_end || p2_start >= p1_start && p2_end <= p1_end
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (p1, p2) = line.split_once(',').unwrap();
            let (p1_start, p1_end) = str_to_u32(p1.split_once('-').unwrap());
            let (p2_start, p2_end) = str_to_u32(p2.split_once('-').unwrap());

            p1_start <= p2_start && p1_end >= p2_start || p2_start <= p1_start && p2_end >= p1_start
        })
        .count()
}

fn part_2_variant(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            let str_to_u3 = |(a, b): (&str, &str)| {
                (a.parse().unwrap()..=b.parse().unwrap()).collect::<BTreeSet<u32>>()
            };
            let (p1, p2) = line.split_once(',').unwrap();

            let t = &str_to_u3(p1.split_once('-').unwrap()) & &str_to_u3(p2.split_once('-').unwrap());
            t.first().cloned()
        })
        .count()
}
