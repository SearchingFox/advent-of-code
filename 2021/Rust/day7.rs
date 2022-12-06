fn main() {
    let tmp = std::fs::read_to_string("../Input/day7.txt").unwrap();
    let input: Vec<u32> = tmp.split(',').map(|x| x.parse().unwrap()).collect();
    
    let part_1: u32 = (1..*input.iter().max().unwrap()).map(|pos| input.iter().map(|y| y.abs_diff(pos)).sum()).min().unwrap();
    let part_2: u32 = (1..*input.iter().max().unwrap()).map(|pos| input.iter().map(|y| (1..=y.abs_diff(pos)).sum::<u32>()).sum()).min().unwrap();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
