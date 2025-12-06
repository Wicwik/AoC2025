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
            if start > end {
                panic!("Range start must not exceed end: {}-{}", start, end);
            }
            (start, end)
        })
        .collect()
}


fn main() {

    let input = include_str!("input.txt");
    let mut sections = input.splitn(2, "\n\n");

    let ranges_block = sections.next().unwrap_or("");

    let mut ranges = parse_ranges(ranges_block);

    ranges.sort_by_key(|&(start, _)| start);

    let mut unique_total: i64 = 0;
    let mut current: Option<(i64, i64)> = None;

    for (start, end) in ranges {
        match current {
            None => current = Some((start, end)),
            Some((cur_start, cur_end)) => {
                if start <= cur_end + 1 {
                    let merged_end = cur_end.max(end);
                    current = Some((cur_start, merged_end));
                } else {
                    unique_total += cur_end - cur_start + 1;
                    current = Some((start, end));
                }
            }
        }
    }

    if let Some((cur_start, cur_end)) = current {
        unique_total += cur_end - cur_start + 1;
    }

    println!("Total unique values: {}", unique_total);

}