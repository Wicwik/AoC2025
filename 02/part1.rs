fn is_concatenated_double(n: i64) -> bool {
    let s = n.to_string();
    let digits = s.trim_start_matches('-');

    if digits.len() % 2 != 0 {
        return false;
    }

    let half = digits.len() / 2;
    &digits[..half] == &digits[half..]
}

fn main() {

    let input = include_str!("input.txt");
    let mut sum: i64 = 0;

    if let Some(line) = input.lines().next() {
        
        for range in line.split(',') {
            if let Some((start, end)) = range.split_once('-') {
                let start: i64 = start.parse().unwrap();
                let end: i64 = end.parse().unwrap();
                
                for n in start..=end {
                    if is_concatenated_double(n) {
                        sum += n;
                    }
                }
            }

        }
    }

    println!("Sum of concatenated doubles: {}", sum);

}