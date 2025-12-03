fn max_two_digit_concat(line: &str) -> Option<i32> {
    let digits: Vec<u32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();


    let mut max_val: i32 = i32::MIN;
    for (i, &first) in digits.iter().enumerate() {
        for &second in digits.iter().skip(i + 1) {
            let candidate = (first * 10 + second) as i32;
            if candidate > max_val {
                max_val = candidate;
            }
        }
    }

    Some(max_val)
}

fn main() {
    let input = include_str!("input.txt");

    let sum = input
        .lines()
        .filter_map(|line| max_two_digit_concat(line))
        .sum::<i32>();

    println!("Sum of max two-digit concatenations: {}", sum);
}