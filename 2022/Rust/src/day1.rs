pub fn part_1(input: String) -> i32 {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i32>().unwrap()).sum::<i32>())
        .max()
        .unwrap()
}

pub fn part_2(input: String) -> i32 {
    let mut output: Vec<i32> = input
        .lines()
        .collect::<Vec<_>>()
        .group_by(|x, _| !x.is_empty())
        .map(|x| x.iter().flat_map(|y| y.parse::<i32>()).sum())
        .collect::<Vec<_>>();

    output.sort();
    output.iter().rev().take(3).sum()
}
