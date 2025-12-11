use std::collections::HashMap;
use std::io;

fn get_all_paths(
    graph: &Vec<Vec<usize>>,
    from: usize,
    to: usize,
    cache: &mut Vec<Option<usize>>,
) -> usize {
    if let Some(paths) = cache[from] {
        paths
    } else if from == to {
        1
    } else {
        let paths = graph[from]
            .iter()
            .map(|&n| get_all_paths(graph, n, to, cache))
            .sum();
        cache[from] = Some(paths);
        paths
    }
}

fn get_node<'a>(
    node_map: &mut HashMap<&'a str, usize>,
    graph: &mut Vec<Vec<usize>>,
    node: &'a str,
) -> usize {
    *node_map.entry(node).or_insert_with(|| {
        graph.push(Vec::new());
        graph.len() - 1
    })
}

fn main() {
    let mut node_map = HashMap::new();
    let mut graph: Vec<Vec<usize>> = Vec::new();
    let start = get_node(&mut node_map, &mut graph, "you");
    let end = get_node(&mut node_map, &mut graph, "out");
    let input = io::read_to_string(io::stdin()).unwrap();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let from = get_node(&mut node_map, &mut graph, &parts[0][..parts[0].len() - 1]);
        for &to_str in parts[1..].iter() {
            let to = get_node(&mut node_map, &mut graph, to_str);
            graph[from].push(to);
        }
    }
    let res = get_all_paths(&graph, start, end, &mut vec![None; graph.len()]);
    println!("{res}");
}
