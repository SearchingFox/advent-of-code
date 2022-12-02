use std::collections::HashMap;
use std::iter::FromIterator;

fn count_frequencies(m: HashMap<(String, bool), u64>, lc: char) -> u64 {
    let mut freq: HashMap<char, u64> = HashMap::new();
    for ((i, k), j) in m {
        if k {
            let t = i.chars().collect::<Vec<_>>();
            *freq.entry(t[0]).or_default() += j;
            *freq.entry(t[1]).or_default() += j;
        }
    }
    *freq.entry(lc).or_default() += 1;

    freq.values().max().unwrap() - freq.values().min().unwrap()
}

fn main() {
    let tmp = std::fs::read_to_string("day14.txt").unwrap();
    let input = tmp.split("\r\n\r\n").collect::<Vec<_>>();

    let seq = input[0];
    let rules: HashMap<String, &str> = HashMap::from_iter(input[1].lines().map(|x| {
        let t = x.split(" -> ").collect::<Vec<_>>();
        (t[0].to_string(), t[1])
    }));
    let lc = seq.chars().last().unwrap();

    let mut m: HashMap<(String, bool), u64> = HashMap::new();
    for i in seq
        .chars()
        .zip(seq.chars().skip(1))
        .map(|(a, b)| [a, b].iter().collect::<String>())
    {
        *m.entry((i, false)).or_default() += 1;
    }

    for s in 0..40 {
        for ((i, k), j) in m.clone() {
            if j > 0 {
                *m.entry((i.to_string(), k)).or_default() -= j;

                let (t0, t1) = (
                    i.chars().nth(0).unwrap().to_string() + rules.get(&i).unwrap(),
                    rules.get(&i).unwrap().to_string() + &i.chars().nth(1).unwrap().to_string(),
                );
                *m.entry((t0, true)).or_default() += j;
                *m.entry((t1, false)).or_default() += j;
            }
        }

        if s == 9 {
            println!("Part 1: {}", count_frequencies(m.clone(), lc));
        }
    }

    println!("Part 2: {}", count_frequencies(m, lc));
}
