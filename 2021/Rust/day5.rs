use std::cmp::{max, min};

#[derive(Debug)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn dots(&self) -> Vec<(u32, u32)> {
        if self.x1 == self.x2 {
            (min(self.y1, self.y2)..=max(self.y1, self.y2))
                .map(|y| (self.x1, y))
                .collect::<Vec<_>>()
        } else if self.y1 == self.y2 {
            (min(self.x1, self.x2)..=max(self.x1, self.x2))
                .map(|x| (x, self.y1))
                .collect::<Vec<_>>()
        } else {
            let a = if self.x1 < self.x2 {
                (self.x1..=self.x2).collect::<Vec<_>>()
            } else {
                (self.x2..=self.x1).rev().collect::<Vec<_>>()
            };
            let b = if self.y1 < self.y2 {
                (self.y1..=self.y2).collect::<Vec<_>>()
            } else {
                (self.y2..=self.y1).rev().collect::<Vec<_>>()
            };
            a.into_iter().zip(b.into_iter()).collect::<Vec<_>>()
        }
    }
}


fn main() {
    let tmp = std::fs::read_to_string("day5.txt").unwrap();
    let lines = tmp.lines().map(|x| {
        let t = x
            .split(" -> ")
            .map(|y| y.split(',').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Line {
            x1: t[0][0].parse::<u32>().unwrap(),
            y1: t[0][1].parse::<u32>().unwrap(),
            x2: t[1][0].parse::<u32>().unwrap(),
            y2: t[1][1].parse::<u32>().unwrap(),
        }
    });
    let part1 = lines
        .clone()
        .filter(|x| x.is_straight())
        .collect::<Vec<_>>();
    let part2 = lines.collect::<Vec<_>>();

    let mut massiv = vec![vec![0; 1000]; 1000];
    let mut result = 0;

    for line in part1 {
        for dot in line.dots() {
            massiv[dot.1 as usize][dot.0 as usize] += 1;
            if massiv[dot.1 as usize][dot.0 as usize] == 2 {
                result += 1
            }
        }
    }
    println!("Part 1: {}", result);

    massiv = vec![vec![0; 1000]; 1000];
    result = 0;

    for line in part2 {
        for dot in line.dots() {
            massiv[dot.1 as usize][dot.0 as usize] += 1;
            if massiv[dot.1 as usize][dot.0 as usize] == 2 {
                result += 1
            }
        }
    }
    println!("Part 2: {}", result);
}
