fn max_twelve_digit_concat(line: &str) -> Option<i64> {
    let digits: Vec<u8> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    const joltage: usize = 12;
    if digits.len() < joltage {
        return None;
    }

    let mut to_remove = digits.len() - joltage;
    let mut stack: Vec<u8> = Vec::with_capacity(digits.len());

    for &digit in &digits {
        while to_remove > 0 {
            if let Some(&last) = stack.last() {
                if last < digit {
                    stack.pop();
                    to_remove -= 1;
                    continue;
                }
            }
            break;
        }
        stack.push(digit);
    }

    stack.truncate(TARGET_LEN);

    let value = stack
        .iter()
        .fold(0_i64, |acc, &d| acc * 10 + d as i64);

    Some(value)
}

fn main() {
    let input = include_str!("input.txt");

    let sum = input
        .lines()
        .filter_map(|line| max_twelve_digit_concat(line))
        .sum::<i64>();

    println!("Sum of max twelve-digit concatenations: {}", sum);
}