use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let tmp = std::fs::read_to_string("day13.txt").unwrap();
    let input: Vec<&str> = tmp.split("\r\n\r\n").collect();

    let folds = input[1]
        .lines()
        .map(|x| {
            let t: Vec<&str> = x.split('=').collect();
            (t[0].chars().last().unwrap(), t[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut dots: HashSet<(i32, i32)> = HashSet::from_iter(input[0].lines().map(|x| {
        let t: Vec<&str> = x.split(',').collect();
        (t[0].parse::<i32>().unwrap(), t[1].parse::<i32>().unwrap())
    }));

    let mut flag = true;
    for f in folds {
        dots = if f.0 == 'y' {
            dots.iter()
                .map(|&(x, y)| if y > f.1 { (x, f.1 * 2 - y) } else { (x, y) })
                .collect()
        } else {
            dots.iter()
                .map(|&(x, y)| if x > f.1 { (f.1 * 2 - x, y) } else { (x, y) })
                .collect()
        };

        if flag {
            println!("Part 1: {}", dots.len());
            flag = false;
        }
    }

    println!("Part 2: ");
    for y in 0..=dots.iter().map(|&(_, y)| y).max().unwrap() {
        for x in 0..=dots.iter().map(|&(x, _)| x).max().unwrap() {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
