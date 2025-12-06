fn parse_ranges(block: &str) -> Vec<(i64, i64)> {
    block
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (start, end) = line
                .split_once('-')
                .expect("Range lines must contain a dash");
            let start = start
                .trim()
                .parse::<i64>()
                .expect("Invalid start value");
            let end = end
                .trim()
                .parse::<i64>()
                .expect("Invalid end value");
            (start, end)
        })
        .collect()
}

fn is_in_ranges(n: i64, ranges: &[(i64, i64)]) -> bool {
    ranges.iter().any(|&(start, end)| n >= start && n <= end)
}

fn main() {

    let input = include_str!("input.txt");
    let mut sections = input.splitn(2, "\n\n");

    let ranges_block = sections.next().unwrap_or("");
    let numbers_block = sections.next().unwrap_or("");

    let ranges = parse_ranges(ranges_block);
    let mut count = 0;

    for line in numbers_block.lines().filter(|line| !line.trim().is_empty()) {
        let value = line.trim().parse::<i64>().expect("Invalid number");
        if is_in_ranges(value, &ranges) {
            count += 1;
        }
        
    }

    println!("Fresh ingredients count: {}", count);

}