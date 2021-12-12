fn main() {
    let tmp = std::fs::read_to_string("day10.txt").unwrap();
    let matching = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
    let pts = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];
    let pts2 = [('(', 1), ('[', 2), ('{', 3), ('<', 4)];

    let mut s = 0;
    let mut ss2: Vec<u64> = Vec::new();

    for line in tmp.lines() {
        let mut stack: Vec<char> = Vec::new();
        let mut is_corrupted = false;

        for c in line.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push(c);
            } else {
                let matching_start = matching.iter().find(|(_, y)| *y == c).unwrap();
                if matching_start.0 != stack.pop().unwrap() {
                    s += pts.iter().find(|(x, _)| *x == c).unwrap().1;
                    is_corrupted = true;
                }
            }
        }
        if !is_corrupted {
            let mut s2 = 0;
            for i in stack.iter().rev() {
                s2 *= 5;
                s2 += pts2.iter().find(|(x, _)| *x == *i).unwrap().1;
            }
            ss2.push(s2);
        }
    }
    ss2.sort();

    println!("Part 1: {}", s);
    println!("Part 2: {}", ss2[ss2.len() / 2 as usize]);
}
