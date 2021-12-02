fn main() {
    let tmp = std::fs::read_to_string("day2_1.txt").unwrap();
    let s = tmp.lines().collect::<Vec<_>>();

    // Part 1
    let mut horiz = 0;
    let mut depth = 0;
    for line in &s {
        match line.to_string().chars().nth(0).unwrap() {
            'f' => { horiz += line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap() }
            'd' => { depth += line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap() }
            'u' => { depth -= line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap() }
            _   => {}
        }
    }
    println!("{}", horiz * depth);

    // Part 2
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in s {
        let x = line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
        match line.to_string().chars().nth(0).unwrap() {
            'f' => { horiz += x;
                     depth += aim * x; }
            'd' => { aim += x }
            'u' => { aim -= x }
            _   => {}
        }
    }
    println!("{}", horiz * depth);
}
