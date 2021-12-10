fn main() {
    let tmp = std::fs::read_to_string("day3_1.txt").unwrap();

    // Part 1
    let mut count_bits: [i32; 12] = [0; 12];
    let mut len = 0;
    for line in tmp.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                count_bits[i] += 1;
            }
        }
        len += 1;
    }

    let mut gamma: Vec<char> = Vec::new();
    let mut epsilon: Vec<char> = Vec::new();
    for i in count_bits {
        if i > len / 2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    println!(
        "Part 1: {}",
        isize::from_str_radix(&gamma.into_iter().collect::<String>(), 2).unwrap()
            * isize::from_str_radix(&epsilon.into_iter().collect::<String>(), 2).unwrap()
    );

    // Part 2
    let mut oxygen = tmp.lines().collect::<Vec<_>>();
    let mut co2 = tmp.lines().collect::<Vec<_>>();
    for i in 0..12 {
        if oxygen.len() > 1 {
            let most_common = if oxygen
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == '1')
                .count()
                >= oxygen.len() / 2
            {
                '1'
            } else {
                '0'
            };
            oxygen.retain(|&x| x.chars().nth(i).unwrap() == most_common);
        }

        if co2.len() > 1 {
            let least_common = if co2
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == '0')
                .count()
                <= co2.len() / 2
            {
                '0'
            } else {
                '1'
            };
            co2.retain(|&x| x.chars().nth(i).unwrap() == least_common);
        }
    }
    println!(
        "Part 2: {}",
        isize::from_str_radix(&oxygen[0], 2).unwrap() * isize::from_str_radix(&co2[0], 2).unwrap()
    )
}
