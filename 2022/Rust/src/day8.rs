pub fn part_1(input: &str) -> usize {
    let t: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    (0..(t.len() - 2) * (t.len() - 2)).filter_map(|index| {
        let i = index / (t.len() - 2) + 1;
        let j = index % (t.len() - 2) + 1;
        let cur = t[i][j];
        ((1..=i).all(|k| t[i - k][j] < cur)
            || (1..t.len() - i).all(|k| t[i + k][j] < cur)
            || (1..=j).all(|k| t[i][j - k] < cur)
            || (1..t.len() - j).all(|k| t[i][j + k] < cur)).then_some(())
    }).count() + 4 * t.len() - 4
}

pub fn part_2(input: &str) -> usize {
    let t: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut c = 0;
    for i in 1..t.len() - 1 {
        for j in 1..t.len() - 1 {
            let cur = t[i][j];
            //let a = ((1..=i).take_while(|k| t[i - k][j] < cur).count() + 1)
            //    * ((1..t.len() - i).take_while(|k| t[i + k][j] < cur).count() + 1)
            //    * ((1..=j).take_while(|k| t[i][j - k] < cur).count() + 1)
            //    * ((1..t.len() - j).take_while(|k| t[i][j + k] < cur).count() + 1);
            //if a > c {
            //    c = a;
            //}
            let mut up = 0;
            let mut down = 0;
            let mut left = 0;
            let mut right = 0;
            for k in 1..=i {
                up += 1;
                if cur <= t[i - k][j] {
                    break;
                }
            }
            for k in 1..t.len() - i {
                down += 1;
                if cur <= t[i + k][j] {
                    break;
                }
            }
            for k in 1..=j {
                left += 1;
                if cur <= t[i][j - k] {
                    break;
                }
            }
            for k in 1..t.len() - j {
                right += 1;
                if cur <= t[i][j + k] {
                    break;
                }
            }
            if up * down * left * right > c {
                c = up * down * left * right;
            }
        }
    }

    c
}
