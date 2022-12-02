use std::collections::HashMap;

fn main() {
    let tmp = std::fs::read_to_string("day7.txt").unwrap();
    let tmp2 = "16,1,2,0,4,2,7,1,2,14";
    let mut input = tmp.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    input.sort();
    let mut m: HashMap<i32, i32> = HashMap::new();
    for i in input.clone() {
        *m.entry(i).or_default() += 1;
    }
    let max = m.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k).unwrap();
    let res:i32 = input.iter().map(|x| (x - max).abs()).sum();
    // m.into_iter().filter(|(_, v)| *v == 2).map(|(k, _)| k).collect::<Vec<_>>();
    println!("{}, {}, {}", res, max, input.iter().sum::<i32>() / input.len() as i32);
}
