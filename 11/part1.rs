use std::collections::HashMap;

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

    let mut queue = vec!["you".to_string()];
    let mut paths = 0;

    while let Some(current) = queue.pop() {
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                queue.push(neighbor.clone());

                if neighbor == "out" {
                    paths += 1;
                }
            }
            
        }
    }

    println!("Total paths from 'you' to 'out': {}", paths);
}