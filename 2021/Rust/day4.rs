fn check_board(b: &Vec<Vec<&str>>) -> bool {
    let horizontal = b.iter().any(|y| y.iter().all(|&x| x == "x"));
    let vertical = (0..b.len()).any(|i| b.iter().filter(|x| x[i] == "x").count() == b.len());
    horizontal || vertical
}

fn main() {
    let tmp = std::fs::read_to_string("day4.txt").unwrap();
    let input: Vec<&str> = tmp.split("\r\n\r\n").collect();

    let numbers = input[0].split(',').map(String::from).collect::<Vec<_>>();
    let mut boards = input[1..]
        .iter()
        .map(|x| {
            x.lines()
                .map(|x| x.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut answer = vec![];
    let mut done_boards: Vec<u32> = Vec::new();
    let part1 = 0;
    let part2 = boards.len() - 1;
    for i in &numbers {
        let mut c = 0;
        for b in &mut boards {
            if !done_boards.contains(&c) {
                for row in 0..5 {
                    for r in 0..5 {
                        if b[row][r] == i {
                            b[row][r] = "x";
                        }
                    }
                }

                if check_board(&b) {
                    if done_boards.len() == part1 || done_boards.len() == part2 {
                        answer.push((b.clone(), i));
                    } else {
                        done_boards.push(c);
                    }
                }
            }
            c += 1;
        }
        if answer.len() == 2 {
            let sum1: u32 = answer[0].0
                .concat()
                .into_iter()
                .filter(|&x| x != "x")
                .map(|x| x.parse::<u32>().unwrap())
                .sum();
            println!("Part 1: {}", sum1 * answer[0].1.parse::<u32>().unwrap());

            let sum2: u32 = answer[1].0
                .concat()
                .into_iter()
                .filter(|&x| x != "x")
                .map(|x| x.parse::<u32>().unwrap())
                .sum();
            println!("Part 2: {}", sum2 * answer[1].1.parse::<u32>().unwrap());
            break;
        }
    }
}
