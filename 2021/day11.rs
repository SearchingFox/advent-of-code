fn modify_adj(inp: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    inp[i][j] = -100;
    for (a, b) in vec![-1, -1, -1, 0, 0, 1, 1, 1]
        .iter()
        .zip(vec![-1, 0, 1, -1, 1, -1, 0, 1].iter())
    {
        let x1 = i as i32 + a;
        let y1 = j as i32 + b;
        if x1 >= 0 && x1 < inp.len() as i32 && y1 >= 0 && y1 < inp[0].len() as i32 {
            inp[x1 as usize][y1 as usize] += 1;
            if inp[x1 as usize][y1 as usize] > 9 {
                modify_adj(inp, x1 as usize, y1 as usize);
            }
        }
    }
}

fn main() {
    let tmp = std::fs::read_to_string("day11.txt").unwrap();
    let mut input = tmp
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let max_row = input.len();
    let max_column = input[0].len();

    let mut step = 0;
    let mut count = 0;
    loop {
        step += 1;

        for i in &mut input {
            // for j in i {
            //     *j += 1;
            // }
            i.iter_mut().for_each(|x| *x += 1);
        }

        for i in 0..max_row {
            for j in 0..max_column {
                if input[i][j] > 9 {
                    modify_adj(&mut input, i, j);
                }
            }
        }

        let mut cur_count = 0;
        for i in &mut input {
            for j in i {
                if *j < 0 {
                    *j = 0;
                    if step <= 100 {
                        count += 1;
                    }
                    cur_count += 1;
                }
            }
        }

        if cur_count == 100 {
            println!("Part 2: {}", step);
            break;
        }
    }

    println!("Part 1: {}", count);
}
