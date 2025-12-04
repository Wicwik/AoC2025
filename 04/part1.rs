fn count_neighbors(grid: &[Vec<char>], row: usize, col: usize, target: char) -> u32 {
    let mut count = 0;

    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr < 0 || nc < 0 {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            if nr >= grid.len() || nc >= grid[nr].len() {
                continue;
            }

            if grid[nr][nc] == target {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell != '@' {
                print!("{}", cell);
                continue;
            }

            let neighbors = count_neighbors(&grid, row_idx, col_idx, '@');

            if neighbors < 4 {
                count += 1;
                print!("X");
            }
            else {
                print!("{}", cell);
            }
            
        }
        println!();
    }
    println!("Count of cells with fewer than 4 '@' neighbors: {}", count);
}