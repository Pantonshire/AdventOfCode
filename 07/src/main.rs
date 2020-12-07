use std::collections::HashMap;
use std::collections::HashSet;

type Graph<'a> = HashMap<&'a str, Vec<(u32, &'a str)>>;

const TARGET: &'static str = "shiny gold";

fn main() {
    let contents = include_str!("../input");
    let lines = contents.split("\n")
        .map(|l| l.trim()).filter(|l| !l.is_empty());
    let (graph, keys) = build_graph(lines)
        .expect("Failed to build graph");
    let contains_gold: u32 = keys.iter()
        .map(|k| search(TARGET, k, &graph, &HashSet::new()))
        .map(|x| if x { 1 } else { 0 })
        .sum();
    println!("Contains shiny gold: {}", contains_gold);
    let gold_contains = count_contains(TARGET, &graph);
    println!("Shiny gold contains: {}", gold_contains);
}

fn build_graph<'a, I: Iterator<Item=&'a str>>(lines: I) -> Option<(Graph<'a>, Vec<&'a str>)> {
    let mut graph = HashMap::new();
    let mut keys = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let s1: Vec<&str> = line.trim().splitn(2, " bags contain ").collect();
        if s1.len() != 2 {
            return None;
        }
        let bag_type = s1[0];
        keys.push(bag_type);
        let mut contains = Vec::new();
        if !s1[1].starts_with("no other") {
            let types = s1[1].strip_suffix(".")?.split(", ");
            for t in types {
                let ts: Vec<&str> = t.splitn(2, " ").collect();
                if ts.len() != 2 {
                    return None;
                }
                let n: u32 = match ts[0].parse(){
                    Ok(x) => x,
                    Err(_) => return None,
                };
                let ct = match ts[1].strip_suffix(" bag") {
                    Some(s) => s,
                    None => match ts[1].strip_suffix(" bags") {
                        Some(s) => s,
                        None => return None,
                    },
                };
                contains.push((n, ct));
            }
        }
        graph.insert(bag_type, contains);
    }
    Some((graph, keys))
}

fn search(target: &str, k: &str, graph: &HashMap<&str, Vec<(u32, &str)>>, visited: &HashSet<&str>) -> bool {
    if visited.contains(k) {
        return false;
    }
    let bs = match graph.get(k) {
        Some(v) => v,
        None => return false,
    };
    for b in bs {
        if b.1.eq(target) {
            return true;
        } else {
            let mut v = visited.clone();
            v.insert(k);
            if search(target, b.1, graph, &v) {
                return true;
            }
        }
    }
    false
}

fn count_contains(k: &str, graph: &HashMap<&str, Vec<(u32, &str)>>) -> u32 {
    match graph.get(k) {
        Some(bs) => bs.iter().map(|b| b.0 * (1 + count_contains(b.1, graph))).sum(),
        None => 0,
    }
}
