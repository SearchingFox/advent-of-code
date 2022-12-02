fn main() {
    let tmp = std::fs::read_to_string("day1_1.txt").unwrap();
    let s = tmp.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    // Part 1
    println!("{}", s.iter().zip(&s[1..]).fold(0, |acc, (b, c)| { if c > b { acc + 1 } else { acc } }));

    // Part 2
    let mut acc = 0;
    for i in 0..s.len() - 3 {
        if s[i..=i+2].iter().sum::<i32>() < s[i+1..=i+3].iter().sum::<i32>() {
            acc += 1;
        }
    }
    println!("{}", acc);
}
