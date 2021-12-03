fn main() {
    let tmp = std::fs::read_to_string("day3_1.txt").unwrap();

    // Part 1
    let mut result: [i32; 12] = [0; 12];
    let mut len = 0;
    for line in tmp.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                result[i] += 1;
            }
        }
        len += 1;
    }
    let gamma = result.iter().map(|x| if x > &(len / 2) { '1' } else { '0' }).collect::<String>();
    let epsilon = gamma.chars().map(|x| if x == '1' { '0' } else { '1' }).collect::<String>();
    println!("{}", isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap());
}
