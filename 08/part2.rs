use std::cmp::Ordering;

fn euclidean_distance(a: (u64, u64, u64), b: (u64, u64, u64)) -> f64 {
    let dx = a.0 as f64 - b.0 as f64;
    let dy = a.1 as f64 - b.1 as f64;
    let dz = a.2 as f64 - b.2 as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn compute_distance_matrix(points: &[(u64, u64, u64)]) -> Vec<Vec<f64>> {
    points
        .iter()
        .map(|&origin| {
            points
                .iter()
                .map(|&target| {
                    if origin == target {
                        0.0
                    } else {
                        euclidean_distance(origin, target)
                    }
                })
                .collect()
        })
        .collect()
}

fn contains(circuit: &[(u64, u64, u64)], point: (u64, u64, u64)) -> bool {
    circuit.iter().any(|&candidate| candidate == point)
}

fn find_circuit_index(
    circuits: &[Vec<(u64, u64, u64)>],
    point: (u64, u64, u64),
) -> Option<usize> {
    circuits
        .iter()
        .enumerate()
        .find_map(|(idx, circuit)| if contains(circuit, point) { Some(idx) } else { None })
}

fn in_same_circuit(
    circuits: &[Vec<(u64, u64, u64)>],
    a: (u64, u64, u64),
    b: (u64, u64, u64),
) -> bool {
    match (find_circuit_index(circuits, a), find_circuit_index(circuits, b)) {
        (Some(i), Some(j)) => i == j,
        _ => false,
    }
}

fn connect_points(
    circuits: &mut Vec<Vec<(u64, u64, u64)>>,
    a: (u64, u64, u64),
    b: (u64, u64, u64),
) {
    match (find_circuit_index(circuits, a), find_circuit_index(circuits, b)) {
        (Some(i), Some(j)) if i == j => {
            // Already connected.
        }
        (Some(i), Some(j)) => {
            // Merge two circuits.
            let (low, high) = if i < j { (i, j) } else { (j, i) };
            let mut merged = circuits.remove(high);
            let target = &mut circuits[low];
            for point in merged.drain(..) {
                if !contains(target, point) {
                    target.push(point);
                }
            }
        }
        (Some(i), None) => {
            if !contains(&circuits[i], b) {
                circuits[i].push(b);
            }
        }
        (None, Some(j)) => {
            if !contains(&circuits[j], a) {
                circuits[j].push(a);
            }
        }
        (None, None) => {
            circuits.push(vec![a, b]);
        }
    }
}

fn find_min_distances(
    distance_matrix: &[Vec<f64>],
    points: &[(u64, u64, u64)],
    circuits: &[Vec<(u64, u64, u64)>],
    limit: usize,
) -> Vec<(usize, usize, f64)> {
    let mut candidates: Vec<(usize, usize, f64)> = Vec::new();

    for (row_idx, row) in distance_matrix.iter().enumerate() {
        for col_idx in (row_idx + 1)..row.len() {
            let a = points[row_idx];
            let b = points[col_idx];

            if in_same_circuit(circuits, a, b) {
                continue;
            }

            let distance = row[col_idx];
            candidates.push((row_idx, col_idx, distance));
        }
    }

    candidates.sort_by(|lhs, rhs| lhs.2.partial_cmp(&rhs.2).unwrap_or(Ordering::Greater));
    candidates.truncate(limit);
    candidates
}

fn main() {
    let input = include_str!("input.txt");

    let points: Vec<(u64, u64, u64)> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<u64> = line
                .split(',')
                .filter_map(|value| value.parse::<u64>().ok())
                .collect();
            if parts.len() == 3 {
                Some((parts[0], parts[1], parts[2]))
            } else {
                None
            }
        })
        .collect();

    println!("Loaded {} points", points.len());

    let distance_matrix = compute_distance_matrix(&points);

    // println!("[");
    // for row in &distance_matrix {
    //     let formatted: Vec<String> = row.iter().map(|distance| format!("{distance:>8.2}")).collect();
    //     println!("  [{}]", formatted.join(","));
    // }
    // println!("]");

    let mut circuits: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    let candidates = find_min_distances(&distance_matrix, &points, &circuits, 10000);

    if candidates.is_empty() {
        println!("No eligible pairs to connect.");
    } else {
        println!("Top {} minimum-distance pairs:", candidates.len());
        for (rank, &(i, j, min_distance)) in candidates.iter().enumerate() {
            let a = points[i];
            let b = points[j];
            if in_same_circuit(&circuits, a, b) {
                println!(
                    "  #{:>2}: ({}, {}, {}) <-> ({}, {}, {}) [distance {:.2}] — already connected",
                    rank + 1,
                    a.0,
                    a.1,
                    a.2,
                    b.0,
                    b.1,
                    b.2,
                    min_distance
                );
                continue;
            }
            println!(
                "  #{:>2}: ({}, {}, {}) <-> ({}, {}, {}) [distance {:.2}]",
                rank + 1,
                a.0,
                a.1,
                a.2,
                b.0,
                b.1,
                b.2,
                min_distance
            );
            connect_points(&mut circuits, a, b);

            
            if circuits.len() == 1 && circuits[0].len() == points.len() {
                println!("All points are now connected in a single circuit.");
                println!(
                    "lastly connected: #{:>2}: ({}, {}, {}) <-> ({}, {}, {}) [distance {:.2}]",
                    rank + 1,
                    a.0,
                    a.1,
                    a.2,
                    b.0,
                    b.1,
                    b.2,
                    min_distance
                );

                println!("X cords multiplication: {}", a.0 * b.0);

                break;
            }
        }
    }

    // if circuits.is_empty() {
    //     println!("No circuits formed yet.");
    // } else {
    //     circuits.sort_by(|lhs, rhs| rhs.len().cmp(&lhs.len()));

    //     for (idx, circuit) in circuits.iter().enumerate() {
    //         println!(
    //             "Circuit {idx}: {}",
    //             circuit
    //                 .iter()
    //                 .map(|point| format!("({}, {}, {})", point.0, point.1, point.2))
    //                 .collect::<Vec<_>>()
    //                 .join(" -> ")
    //         );
    //     }

    //     let product_top3 = circuits
    //         .iter()
    //         .take(3)
    //         .fold(1u128, |acc, circuit| acc * circuit.len() as u128);

    //     println!(
    //         "Product of top {} circuit sizes: {}",
    //         circuits.len().min(3),
    //         product_top3
    //     );
    // }
}