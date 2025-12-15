#[derive(Debug)]
struct Machine {
    indicators: u32,
    buttons: Vec<Vec<u32>>,
}

fn subsets<T: Copy>(set: &[T]) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = Vec::new();
    for count in 0..=set.len() {
        subsets.extend(get_combinations(set, count));
    }
    subsets
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

    let indicators: u32 = indicators_str
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            if i >= 32 {
                panic!("Indicator length exceeds 32 bits: {}", indicators_str.len());
            }
            match ch {
                '#' => 1 << i,
                '.' => 0,
                other => panic!("Invalid indicator character: {}", other),
            }
        })
        .sum();

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

    Machine {
        indicators,
        buttons,
    }
}


fn find_end_state(machine: &Machine) -> usize {
    let binary_buttons = get_binary_buttons(&machine.buttons);
    for subset in subsets(&binary_buttons) {
        if subset.iter().fold(0, |a, &b| a ^ b) == machine.indicators {
            return subset.len()
        }
    }
    unreachable!()
}

fn main() {
    let input = include_str!("input.txt");

    let machines: Vec<Machine> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_machine)
        .collect();

    let button_count = machines.iter().map(|m| find_end_state(m)).sum::<usize>();
    println!("Button count: {}", button_count);
}