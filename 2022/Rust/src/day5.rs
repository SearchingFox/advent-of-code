use std::collections::BTreeMap;

fn parse_containers(i: &str) -> BTreeMap<u8, Vec<char>> {
    i.lines()
        .flat_map(|line| {
            line.chars()
                .enumerate()
                .flat_map(|(i, c)| c.is_ascii_alphabetic().then_some(((i / 4 + 1) as u8, c)))
        })
        .fold(BTreeMap::new(), |mut acc, (i, c)| {
            acc.entry(i).and_modify(|x| x.push(c)).or_insert(vec![c]);
            acc
        })
}

pub fn part_1(input: &str) -> String {
    let (t1, t2) = input.split_once("\r\n\r\n").unwrap();
    let mut containers = parse_containers(t1);

    for line in t2.lines() {
        let t: Vec<&str> = line.split(' ').collect();
        let size: usize = t[1].parse().unwrap();
        let from: u8 = t[3].parse().unwrap();
        let to: u8 = t[5].parse().unwrap();
        let from_value = containers.get(&from).unwrap().clone();

        containers.insert(from, from_value[size..].to_vec());
        containers.insert(
            to,
            from_value[..size]
                .iter()
                .rev()
                .chain(containers.get(&to).unwrap())
                .cloned()
                .collect(),
        );
    }

    containers
        .values()
        .flat_map(|v| v.first())
        .collect::<String>()
}

pub fn part_2(input: &str) -> String {
    let (t1, t2) = input.split_once("\r\n\r\n").unwrap();
    let mut containers = parse_containers(t1);

    for line in t2.lines() {
        let t: Vec<&str> = line.split(' ').collect();
        let size: usize = t[1].parse().unwrap();
        let from: u8 = t[3].parse().unwrap();
        let to: u8 = t[5].parse().unwrap();
        let from_value = containers.get(&from).unwrap().clone();

        containers.insert(from, from_value[size..].to_vec());
        containers.insert(
            to,
            [&from_value[..size], containers.get(&to).unwrap()].concat(),
        );
    }

    containers
        .values()
        .flat_map(|v| v.first())
        .collect::<String>()
}
