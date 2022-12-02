pub fn part_1(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let t: Vec<char> = line.chars().collect();
            match t[2] as i32 - t[0] as i32 {
                21 | 24 => 6 + t[2] as i32 - 87,
                22 | 25 => t[2] as i32 - 87,
                23 => 3 + t[2] as i32 - 87,
                _ => panic!("It can't be!"),
            }
        })
        .sum()
}

pub fn part_2(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let t: Vec<char> = line.chars().collect();
            match t[2] {
                'X' => t[0] as i32 % 3 + 1,
                'Y' => 3 + t[0] as i32 - 64,
                'Z' => 6 + (t[0] as i32 - 1) % 3 + 1,
                _ => panic!("It can't be!"),
            }
        })
        .sum()
}
