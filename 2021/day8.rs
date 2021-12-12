fn main() {
    let tmp = std::fs::read_to_string("day8.txt").unwrap();
    let input = tmp
        .lines()
        .map(|line| {
            line.split(" | ")
                .map(|x| x.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        input
            .iter()
            .map(|x| x[1]
                .iter()
                .filter(|y| [2, 3, 4, 7].contains(&y.len()))
                .count())
            .sum::<usize>()
    );
}
