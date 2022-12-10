pub fn part_1(input: &str) -> i32 {
    let mut x = 1;
    let mut xl = 1;
    let mut c = 1;
    let mut sum = 0;
    let mut dd = vec![220, 180, 140, 100, 60, 20];

    for line in input.lines() {
        if dd.contains(&c) {
            sum += c * x;
            dd.pop();
        }
        if dd.iter().any(|&x| x == c - 1) {
            sum += (c - 1) * xl;
            dd.pop();
        }

        if let Some((_, v)) = line.split_once(' ') {
            xl = x;
            x += v.parse::<i32>().unwrap();
            c += 2;
        } else {
            c += 1
        };
    }

    sum
}

pub fn part_2(input: &str) {
    let mut x = 1;
    let mut prev_add = 0;
    let mut cursor = 0;
    let mut flag = true;
    let mut commands = input.lines();
    let mut command = commands.next();

    while command.is_some() {
        if flag {
            x += prev_add;
            if let Some((_, v)) = command.unwrap().split_once(' ') {
                prev_add = v.parse::<i32>().unwrap();
                flag = false;
            } else {
                prev_add = 0;
            }
            command = commands.next();
        } else {
            flag = true;
        }

        if cursor % 40 == 0 {
            println!();
            cursor = 0;
        }

        if [x - 1, x, x + 1].contains(&cursor) {
            print!("#");
        } else {
            print!(".");
        }

        cursor += 1;
    }

    println!();
}
