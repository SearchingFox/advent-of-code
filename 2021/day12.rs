use std::collections::HashMap;

fn dfs(cur: String, inp: HashMap<String, Vec<String>>, visited: Vec<String>) -> Vec<Vec<String>> {
    if cur == "end" {
        vec![vec!["end".to_string()]]
    } else {
        let mut result: Vec<Vec<String>> = Vec::new();
        for i in &inp[&cur] {
            let mut vis = visited.clone();
            if !vis.contains(&i) {
                if i.chars().next().unwrap().is_lowercase() {
                    vis.push(i.to_string());
                }

                for mut j in dfs(i.clone(), inp.clone(), vis) {
                    j.push(cur.clone());
                    result.push(j);
                }
            }
        }
        result
    }
}
fn main() {
    let tmp = std::fs::read_to_string("day12.txt").unwrap();
    let graph = tmp1
        .lines()
        .filter_map(|x| {
            let t = x.split('-').collect::<Vec<_>>();
            // if t[0] != "start"
            //     && t[1] != "end"
            //     && t[0].chars().next().unwrap().is_lowercase()
            //     && t[1].chars().next().unwrap().is_lowercase()
            // {
            //     None
            // } else {
                Some((t[0].to_string(), t[1].to_string()))
            //}
        })
        .collect::<Vec<_>>();
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for (i, j) in graph {
        if j != "start" {
            m.entry(i.clone()).or_default().push(j.clone());
        }
        if i != "start" && j != "end" {
            m.entry(j).or_default().push(i);
        }
    }
    let mut a: Vec<String> = Vec::new();
    let mut b = dfs3("start".to_string(), &m, &b, 0, 0);
    println!("Part 1: {}", b.len());
