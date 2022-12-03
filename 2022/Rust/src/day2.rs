pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let t: Vec<u8> = line.chars().map(|x| x as u8).collect();
            (match t[2] - t[0] {
                21 | 24 => 6 + t[2] - 87,
                22 | 25 => t[2] - 87,
                23 => 3 + t[2] - 87,
                _ => panic!("It can't be!"),
            }) as u32
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let t: Vec<u8> = line.chars().map(|x| x as u8).collect();
            (match t[2] {
                88 => t[0] % 3 + 1,
                89 => 3 + t[0] - 64,
                90 => 6 + (t[0] - 1) % 3 + 1,
                _ => panic!("It can't be!"),
            }) as u32
        })
        .sum()
}
