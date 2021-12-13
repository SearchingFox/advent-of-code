fn main() {
    let tmp = std::fs::read_to_string("day6.txt").unwrap();

    let mut arr = vec![vec![0; 9]; 257];
    for i in tmp.split(',') {
        arr[0][i.parse::<usize>().unwrap()] += 1;
    }
    for i in 1..257 {
        for j in 1..9 {
            arr[i][j-1] += arr[i-1][j];
        }
        arr[i][6] += arr[i-1][0];
        arr[i][8] = arr[i-1][0];
    }

    println!("Part 1: {}", arr[80].iter().sum::<u64>());
    println!("Part 2: {}", arr[256].iter().sum::<u64>());
}
