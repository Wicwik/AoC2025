fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn square_area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let width = (a.0 - b.0).abs() + 1;
    let height = (a.1 - b.1).abs() + 1;
    width * height
}


fn intersects(edges: &Vec<((i64, i64), (i64, i64))>, a: (i64, i64), b: (i64, i64)) -> bool {
    for edge in edges.iter() {
        let (edge_x_min, edge_x_max) = if edge.0 .0 < edge.1 .0 {
            (edge.0 .0, edge.1 .0)
        } else {
            (edge.1 .0, edge.0 .0)
        };

        let (edge_y_min, edge_y_max) = if edge.0 .1 < edge.1 .1 {
            (edge.0 .1, edge.1 .1)
        } else {
            (edge.1 .1, edge.0 .1)
        };

        if a.0 < edge_x_max && b.0 > edge_x_min &&
           a.1 < edge_y_max && b.1 > edge_y_min {
            return true;
        }
    }
    false
}

fn main() {
    let input = include_str!("input.txt");

    let red_points: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let values: Vec<i64> = line
                .split(|c: char| c == ',')
                .map(|token| token.parse::<i64>().expect("Invalid integer"))
                .collect();
            (values[0], values[1])
        })
        .collect();
    
    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::new();
    
    edges.push((red_points[0], red_points[red_points.len()-1]));
    
    for i in 0..red_points.len()-1 {
        edges.push((red_points[i], red_points[i+1]));
    }

    let mut maximum_distance = 0;

    for from_i in 0..red_points.len()-1 {
        for to_i in from_i..red_points.len() {
            let from = red_points[from_i];
            let to = red_points[to_i];
            let (min_x, max_x) = if from.0 < to.0 { (from.0, to.0) } else { (to.0, from.0) };
            let (min_y, max_y) = if from.1 < to.1 { (from.1, to.1) } else { (to.1, from.1) };

            let dist = manhattan_distance(from, to);

            if dist*dist > maximum_distance {
                if !intersects(&edges, (min_x, min_y), (max_x, max_y)) {
                    let area = square_area(from, to);
                    if area > maximum_distance {
                        maximum_distance = area;
                    }
                }
            }
        }
    
    }

    println!("{}", maximum_distance);

}