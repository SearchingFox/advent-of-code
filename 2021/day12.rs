use std::collections::HashMap;

fn dfs<'a>(
    cur: &'a str,
    inp: &HashMap<&'a str, Vec<&'a str>>,
    visited: &HashMap<&'a str, u32>,
    depth: u32,
) -> u32 {
    if cur == "end" {
        1
    } else {
        let mut result = 0;
        for i in &inp[&cur] {
            let mut vis = visited.clone();
            if vis.get(i).unwrap_or(&0) < &depth
                && vis.iter().filter(|&(a, b)| a != i && b == &2).count() <= 1
            {
                if i.chars().next().unwrap().is_lowercase() {
                    *vis.entry(i).or_default() += 1;
                }
                result += dfs(i.clone(), &inp, &vis, depth)
            }
        }
        result
    }
}

fn main() {
    let input = std::fs::read_to_string("day12.txt").unwrap();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for g in input.lines().map(|x| x.split('-').collect::<Vec<_>>()) {
        if g[1] != "start" {
            graph.entry(g[0]).or_default().push(g[1]);
        }
        if g[0] != "start" && g[1] != "end" {
            graph.entry(g[1]).or_default().push(g[0]);
        }
    }

    println!("Part 1: {}", dfs("start", &graph, &HashMap::new(), 1));
    println!("Part 2: {}", dfs("start", &graph, &HashMap::new(), 2));
}
