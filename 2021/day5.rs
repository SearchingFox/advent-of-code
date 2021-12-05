#[derive(Debug)]
struct Line {
    start: (u32, u32),
    end: (u32, u32)
}

impl Line {
    fn is_straight(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }
}

fn belongs((x, y): (u32, u32), l: &Line) -> bool {
    std::cmp::min(l.start.0, l.end.0) <= x && x <= std::cmp::max(l.start.0, l.end.0) && std::cmp::min(l.start.1, l.end.1) <= y && y <= std::cmp::max(l.start.1, l.end.1)
}

fn main() {
    let tmp = std::fs::read_to_string("day5.txt").unwrap();
    let input = tmp.lines().map(|x| {
        let t = x.split(&[" -> ", ","][..]).map(|y| y.split(',').collect::<Vec<_>>()).collect::<Vec<_>>();
        Line { start: (t[0][0].parse::<u32>().unwrap(), t[0][1].parse::<u32>().unwrap()),
               end: (t[1][0].parse::<u32>().unwrap(), t[1][1].parse::<u32>().unwrap()) }
    }).filter(|x| x.is_straight()).collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..=1000 {
        for j in 0..=1000 {
            let mut count = 0;
            for line in &input {
                if belongs((i, j), line) {
                    count += 1
                }
            }
            if count > 1 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
