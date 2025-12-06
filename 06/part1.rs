fn compute_column_values(input: &str) -> Vec<i128> {
    let mut lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let ops_line = lines
        .pop()
        .expect("Input must contain at least one line of operators");

    let operators: Vec<char> = ops_line
        .split_whitespace()
        .map(|token| token.chars().next().expect("Operator token cannot be empty"))
        .collect();

    let columns = operators.len();

    let mut results: Vec<i128> = operators
        .iter()
        .map(|op| match op {
            '+' => 0,
            '*' => 1,
            _ => panic!("Unsupported operator: {}", op),
        })
        .collect();

    for line in lines.iter() {
        let values: Vec<i128> = line
            .split_whitespace()
            .map(|token| token.parse::<i128>().expect("Invalid integer in input"))
            .collect();


        for col in 0..columns {
            match operators[col] {
                '+' => results[col] += values[col],
                '*' => results[col] *= values[col],
                _ => unreachable!(),
            }
        }
    }

    results
}

fn main() {
    let input = include_str!("input.txt");
    let total: i128 = compute_column_values(input).iter().sum();

    println!("Total: {}", total);
}