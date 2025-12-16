#[derive(Debug)]
struct Shape {
    pattern: Vec<String>,
    count: usize,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    requirements: Vec<u32>,
}

fn parse_input(raw: &str) -> (Vec<Shape>, Vec<Region>) {
    let mut lines = raw.lines().peekable();
    let mut shapes = Vec::new();

    while let Some(line) = lines.peek() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            lines.next();
            continue;
        }

        if trimmed.contains('x') {
            break;
        }

        let _ = lines.next().unwrap();

        let mut pattern = Vec::new();
        let mut count = 0;  
        while let Some(&next_line) = lines.peek() {
            if next_line.trim().is_empty() {
                lines.next();
                break;
            }
            if next_line.contains(':') {
                break;
            }
            let line_str = lines.next().unwrap().to_string();
            count += line_str.chars().filter(|&c| c == '#').count();
            pattern.push(line_str);
        }

        shapes.push(Shape { pattern, count });
    }

    let mut regions = Vec::new();

    while let Some(line) = lines.next() {
        println!("Parsing region line: {}", line);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (size_part, counts_part) = trimmed
            .split_once(':')
            .expect("region line must contain ':'");

        let (width, height) = size_part
            .split_once('x')
            .map(|(w, h)| {
                let width = w.trim().parse::<usize>().expect("invalid width");
                let height = h.trim().parse::<usize>().expect("invalid height");
                (width, height)
            })
            .expect("region size must contain 'x'");

        let requirements = counts_part
            .split_whitespace()
            .map(|value| value.parse::<u32>().expect("invalid requirement"))
            .collect();

        regions.push(Region {
            width,
            height,
            requirements,
        });
    }

    (shapes, regions)
}

fn main() {
    let input = include_str!("input.txt");
    let (shapes, regions) = parse_input(input);

    println!("Loaded {} shapes", shapes.len());
    for shape in &shapes {
        for row in &shape.pattern {
            println!("{row}");
        }
        println!("Count of '#': {}", shape.count);
        println!();
    }

    println!("Loaded {} regions", regions.len());
    let mut region_count = 0;
    for region in &regions {
        println!(
            "Region {}x{} requires {:?}",
            region.width, region.height, region.requirements
        );

        let total_required: u32 = region.requirements.iter().enumerate().map(|(c, r)| r * (shapes[c].count) as u32).sum();
        
        println!("Total '#' required: {}", total_required);

        if (total_required as usize) < region.width * region.height {
            region_count += 1;
        }
        
    }
    println!("Number of regions that cannot fit required shapes: {}", region_count);
}