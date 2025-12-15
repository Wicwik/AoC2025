#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<u32>>,
    joltages: Vec<i32>,
}

fn subsets<T: Copy>(set: &[T]) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = Vec::new();
    for count in 0..=set.len() {
        subsets.extend(get_combinations(set, count));
    }
    subsets
}

fn fewest_joltage_presses(machine: &Machine) -> usize {
    let binary_buttons = get_binary_buttons(&machine.buttons);
    let subset_xors: Vec<_> = subsets(&binary_buttons)
        .iter()
        .map(|subset| (subset.to_owned(), subset.iter().fold(0, |a, &b| a ^ b)))
        .collect();
    fewest_joltage_presses_recur(&subset_xors, &machine.joltages).unwrap()
}

fn fewest_joltage_presses_recur(subset_xors: &[(Vec<u32>, u32)], joltages: &[i32]) -> Option<usize> {
    if joltages.iter().all(|&j| j == 0) {
        return Some(0);
    }
    let binary_joltages = get_binary_joltages(joltages);
    let mut best = None;
    for (subset, xor) in subset_xors {
        if *xor == binary_joltages {
            let new_joltages = get_new_joltages(joltages, &subset);
            if new_joltages.iter().all(|&j| j >= 0) {
                let press_count = fewest_joltage_presses_recur(
                    subset_xors, &new_joltages
                ).map(|c| subset.len() + 2 * c);
                best = best.min(press_count).or(best).or(press_count);
            }
        }
    }
    best
}

fn get_new_joltages(joltages: &[i32], subset: &[u32]) -> Vec<i32> {
    let mut new_joltages = Vec::new();
    let mut mask = 1;
    for &joltage in joltages {
        new_joltages.push((joltage - subset.iter().filter(|&b| b & mask != 0).count() as i32) / 2);
        mask <<= 1;
    }
    new_joltages
}

fn get_binary_joltages(joltages: &[i32]) -> u32 {
    joltages
        .iter()
        .enumerate()
        .map(|(i, j)| ((1 << i) * (j % 2)) as u32)
        .sum()
}


fn get_combinations<T: Copy>(set: &[T], count: usize) -> Vec<Vec<T>> {
    if count == 0 {
        vec![Vec::new()]
    } else {
        set[..set.len() - count + 1]
            .iter()
            .enumerate()
            .flat_map(
                |(i, &t)|
                get_combinations(&set[i+1..], count - 1)
                    .iter()
                    .map(|c| { let mut c1 = c.clone(); c1.push(t); c1 })
                    .collect::<Vec<Vec<T>>>()
            ).collect()
    }
}

fn get_binary_buttons(buttons: &[Vec<u32>]) -> Vec<u32> {
    buttons
        .iter()
        .map(|b| b.iter().map(|n| 1u32 << n).sum())
        .collect()
}

fn parse_machine(line: &str) -> Machine {
    let indicators_end = line.find(']').expect("Missing indicators closing bracket");
    let indicators_str = &line[1..indicators_end];
    let indicator_len = indicators_str.len();

    let mut rest = &line[indicators_end + 1..];
    rest = rest.trim_start();

    let buttons_str_end = rest.rfind(')').expect("Missing buttons closing parenthesis");
    let buttons_str = &rest[..=buttons_str_end];
    let joltage_start = buttons_str_end + 1;
    let joltage_str = rest[joltage_start..].trim();

    let buttons: Vec<Vec<u32>> = buttons_str
        .split_whitespace()
        .filter(|chunk| chunk.starts_with('(') && chunk.ends_with(')'))
        .map(|chunk| {
            chunk[1..chunk.len() - 1]
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let value = s.parse::<u32>().expect("Invalid button value");
                    if value as usize >= indicator_len {
                        panic!(
                            "Button index {} exceeds indicator length {}",
                            value, indicator_len
                        );
                    }
                    value
                })
                .collect()
        })
        .collect();

    let joltages: Vec<i32> = joltage_str
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<i32>().expect("Invalid joltage value"))
        .collect();

    Machine {
        buttons,
        joltages,
    }
}


fn main() {
    let input = include_str!("input.txt");

    let machines: Vec<Machine> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_machine)
        .collect();

    let button_count = machines.iter().map(|m| fewest_joltage_presses(m)).sum::<usize>();
    println!("Button count: {}", button_count);
}