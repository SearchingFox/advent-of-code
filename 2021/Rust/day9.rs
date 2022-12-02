fn get_adj((i, j): (usize, usize), max_row: usize, max_column: usize) -> Vec<(usize, usize)> {
    let mut adj: Vec<(usize, usize)> = Vec::new();

    if i > 0 {
        adj.push((i - 1, j));
    }
    if i < max_row - 1 {
        adj.push((i + 1, j));
    }
    if j > 0 {
        adj.push((i, j - 1));
    }
    if j < max_column - 1 {
        adj.push((i, j + 1));
    }

    adj
}

fn main() {
    let tmp = std::fs::read_to_string("day9.txt").unwrap();
    let input = tmp
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let max_row = input.len();
    let max_column = input[0].len();

    let mut s = 0;
    let mut s2: Vec<usize> = Vec::new();

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if get_adj((i, j), max_row, max_column)
                .into_iter()
                .all(|(a, b)| input[i][j] < input[a][b])
            {
                // Part 1
                s += input[i][j] + 1;

                // Part 2
                let mut basin = vec![(i, j)];
                let mut temp = vec![(i, j)];
                while temp.len() > 0 {
                    temp = temp
                        .into_iter()
                        .flat_map(|x| get_adj(x, max_row, max_column))
                        .filter(|x| input[x.0][x.1] < 9 && !basin.contains(x))
                        .collect::<Vec<_>>();
                    basin.extend(temp.iter());
                }
                basin.sort();
                basin.dedup();
                s2.push(basin.len());
            }
        }
    }

    s2.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {}", s);
    println!("Part 2: {}", s2[0] * s2[1] * s2[2]);
}
