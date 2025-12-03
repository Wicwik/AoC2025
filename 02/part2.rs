fn is_concatenated_double(n: i64) -> bool {
    let digits = n.to_string();

    let len = digits.len();
    if len < 2 {
        return false;
    }

    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }

        let repetitions = len / pattern_len;
        if repetitions < 2 {
            continue;
        }

        let pattern = &digits[..pattern_len];
        if digits
            .as_bytes()
            .chunks(pattern_len)
            .all(|chunk| chunk == pattern.as_bytes())
        {
            return true;
        }
    }

    false
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