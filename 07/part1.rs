use std::collections::{HashMap, HashSet};

fn collect_splits(grid: &[Vec<char>], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let rows = grid.len();
    if rows == 0 {
        return HashSet::new();
    }

    let cols = grid[0].len() as isize;
    let mut memo: HashMap<(usize, isize), HashSet<(usize, usize)>> = HashMap::new();

    fn dfs(
        grid: &[Vec<char>],
        rows: usize,
        cols: isize,
        start_row: usize,
        start_col: isize,
        memo: &mut HashMap<(usize, isize), HashSet<(usize, usize)>>,
    ) -> HashSet<(usize, usize)> {
        if start_col < 0 || start_col >= cols {
            return HashSet::new();
        }

        if let Some(cached) = memo.get(&(start_row, start_col)) {
            return cached.clone();
        }

        let mut row = start_row;
        while row + 1 < rows {
            row += 1;
            let cell = grid[row][start_col as usize];

            if cell == '^' {
                let mut splits: HashSet<(usize, usize)> = HashSet::new();
                splits.insert((row, start_col as usize));

                let left = dfs(grid, rows, cols, row, start_col - 1, memo);
                splits.extend(left);

                let right = dfs(grid, rows, cols, row, start_col + 1, memo);
                splits.extend(right);

                memo.insert((start_row, start_col), splits.clone());
                return splits;
            }
        }

        memo.insert((start_row, start_col), HashSet::new());
        HashSet::new()
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

    let splits = collect_splits(&grid, start.unwrap());
    println!("Total splits: {}", splits.len());
}