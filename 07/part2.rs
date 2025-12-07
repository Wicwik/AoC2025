use std::collections::HashMap;

fn count_paths(grid: &[Vec<char>], start: (usize, usize)) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }

    let cols = grid[0].len() as isize;
    let mut memo: HashMap<(usize, isize), usize> = HashMap::new();

    fn dfs(
        grid: &[Vec<char>],
        rows: usize,
        cols: isize,
        start_row: usize,
        start_col: isize,
        memo: &mut HashMap<(usize, isize), usize>,
    ) -> usize {
        if start_col < 0 || start_col >= cols {
            return 1;
        }

        if let Some(&cached) = memo.get(&(start_row, start_col)) {
            return cached;
        }

        let mut row = start_row;
        while row + 1 < rows {
            row += 1;
            let cell = grid[row][start_col as usize];
            if cell == '^' {
                let left = dfs(grid, rows, cols, row, start_col - 1, memo);
                
                let right = dfs(grid, rows, cols, row, start_col + 1, memo);
               
                let total = left + right;
                
                memo.insert((start_row, start_col), total);
                return total;
            }
        }

        memo.insert((start_row, start_col), 1);
        1
    }

    dfs(grid, rows, cols, start.0, start.1 as isize, &mut memo)
}

fn main() {
    let input = include_str!("input.txt");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = None;

    for (row, line) in grid.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            match ch {
                'S' => start = Some((row, col)),
                _ => {}
            }
        }
    }

    let total_paths = count_paths(&grid, start.unwrap());
    println!("Total paths: {}", total_paths);
}