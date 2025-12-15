use std::collections::HashMap;

fn dfs(
    device: String,
    fft: bool,
    dac: bool,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if device == "out" {
        return if fft && dac { 1 } else { 0 };
    }

    let mut paths_count = 0;

    let next_fft = fft || device == "fft";
    let next_dac: bool = dac || device == "dac";

    for next_device in graph.get(&device).unwrap() {
        if let Some(m) = memo.get(&(next_device.to_string(), next_fft, next_dac)) {
            paths_count += m;
        } else {
            let r = dfs(next_device.to_string(), next_fft, next_dac, graph, memo);
            paths_count += r;
            memo.insert((next_device.clone(), next_fft, next_dac), r);
        }
    }

    paths_count
}

fn main() {
    let input = include_str!("input.txt");

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        if let Some((node, neighbors)) = line.split_once(':') {
            let node = node.trim().to_string();
            let targets: Vec<String> = neighbors
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .collect();
            graph.insert(node, targets);
        }
    }

    println!("Loaded {} entries", graph.len());
    for (node, neighbors) in &graph {
        println!("{} -> {:?}", node, neighbors);
    }

    let mut queue = vec!["svr".to_string()];
    let mut paths = 0;

    
    println!("Total paths from 'svr' to 'out' containing 'fft' and 'dac': {}", dfs("svr".to_string(), false, false, &graph, &mut HashMap::new()));
}