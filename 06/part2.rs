fn compute(numbers: Vec<i128>, operator: char) -> i128 {
    let result: i128;

    match operator {
            '+' => result = numbers.iter().sum(),
            '*' => result = numbers.iter().product(),
            _ => panic!("Unsupported operator: {}", operator),
        };

    result
}

fn parse_number(chars: &[char]) -> i128 {
    let num_str: String = chars.iter().collect();
    num_str.parse::<i128>().expect("Invalid integer in input")
}

fn compute_column_aggregates(input: &str) -> Vec<i128> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let n_cols = lines
        .first()
        .expect("Input must contain at least one line")
        .len();

    // println!("Lines: {:?}", lines);

    let mut numbers : Vec<i128> = Vec::new();
    let mut results : Vec<i128> = Vec::new();

    // println!("Number of columns: {}  Number of rows: {}", n_cols, lines.len());

    for i in (0..n_cols).rev() {
        let mut number : Vec<char> = Vec::new();
        for j in 0..lines.len() {
            let col_char = lines[j][i];

            // println!("Processing char: {}", col_char);
            
            match col_char {
                '+' | '*' => {
                    if !number.is_empty() {
                        numbers.push(parse_number(&number));
                        number.clear();
                        // println!("Parsed number: {}", numbers.last().unwrap());
                    }

                    let col_result = compute(numbers.clone(), col_char);
                    // println!("Column {}: operator {} -> result {}", i, col_char, col_result);
                    numbers.clear();
                    results.push(col_result);
                },
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    number.push(col_char);
                }
                _ => continue,
            }
        }

        if !number.is_empty() {
            numbers.push(parse_number(&number));
            number.clear();
            // println!("Parsed number: {}", numbers.last().unwrap());
        }
    }


    results
}

fn main() {
    let input = include_str!("input.txt");
    let column_totals = compute_column_aggregates(input);

    let total: i128 = column_totals.iter().sum();
    println!("Total: {}", total);
}