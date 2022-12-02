// fn min_cost(i, j) {
//     i

fn main() {
    // let tmp = std::fs::read_to_string("day15.txt").unwrap();
    let tmp1 = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    let input = tmp1.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    println!("{:?}", input);

}
