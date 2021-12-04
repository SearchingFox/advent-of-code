fn check_board(b: &Vec<Vec<&str>>) -> bool {
    let hor = b.iter().any(|y| y.iter().all(|&x| x == "x"));
    let mut ver = false;
    for i in 0..5 {
        let mut count = 0;
        for k in 0..5 {
            if b[k][i] == "x" {
                count += 1;
            }
        }
        if count == 5 {
            ver = true;
            break;
        }
    }
    if hor || ver {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let tmp = std::fs::read_to_string("day4.txt").unwrap();
    let input: Vec<&str> = tmp.split("\r\n\r\n").collect();
    let numbers = input[0].split(',').collect::<Vec<_>>();
    // println!("{:?}", numbers);
    let mut boards = input[1..]
        .iter()
        .map(|x| {
            x.lines()
                .map(|x| x.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // println!("{:?}", boards);

    let mut flag = None;
    for i in numbers {
        for b in &mut boards {
            for row in 0..5 {
                for r in 0..5 {
                    if b[row][r] == i {
                        b[row][r] = "x";
                    }
                }
            }
            if check_board(&b) {
                println!("true");
                flag = Some((b.clone(), i));
            }
        }
        if let Some((t, i)) = flag {
            let mut sum = 0;
            for row in t {
                for col in row {
                    if col != "x" {
                        sum += col.parse::<u32>().unwrap();
                    }
                }
            }
            println!("{}", sum * i.parse::<u32>().unwrap());
            break;
        }
    }
    // println!("{:?}", boards);
}
