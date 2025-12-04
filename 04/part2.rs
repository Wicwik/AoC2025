use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn remove_rolls(grid: &mut [Vec<char>], to_remove: &[(usize, usize)]) {
    for &(row, col) in to_remove {
        grid[row][col] = '.';
    }
}

fn visualize_step(step: usize, grid: &[Vec<char>], to_remove: &[(usize, usize)]) -> std::io::Result<()> {
    let to_remove_set: HashSet<(usize, usize)> = to_remove.iter().copied().collect();
    if grid.is_empty() {
        return Ok(());
    }

    let cell_size = 10;
    let height = grid.len() * cell_size;
    let width = grid[0].len() * cell_size;
    let mut path = PathBuf::from("steps");
    fs::create_dir_all(&path)?;
    path.push(format!("step_{:03}.ppm", step));
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;

    for (row_idx, row) in grid.iter().enumerate() {
        let mut row_colors: Vec<(u8, u8, u8)> = Vec::with_capacity(row.len());
        for (col_idx, cell) in row.iter().enumerate() {
            let color = if to_remove_set.contains(&(row_idx, col_idx)) {
                (220, 20, 60)
            } else if *cell == '@' {
                (0, 0, 0)
            } else {
                (245, 245, 245)
            };
            row_colors.push(color);
        }

        for _ in 0..cell_size {
            for &(r, g, b) in &row_colors {
                for _ in 0..cell_size {
                    writer.write_all(format!("{} {} {} ", r, g, b).as_bytes())?;
                }
            }
            writer.write_all(b"\n")?;
        }
    }

    writer.flush()?;
    println!("Wrote {}", path.display());
    Ok(())
}

fn collect_frames(frames_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut frames: Vec<PathBuf> = fs::read_dir(frames_dir)
        .map_err(|err| format!("failed to read {}: {}", frames_dir.display(), err))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("ppm") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    frames.sort();

    if frames.is_empty() {
        return Err(format!("no frames found in {}", frames_dir.display()))
    }

    Ok(frames)
}

fn attempt_convert(tool: &str, frames: &[PathBuf], output: &Path) -> Result<(), String> {
    let mut command = if tool == "magick" {
        let mut cmd = Command::new(tool);
        cmd.arg("convert");
        cmd
    } else {
        Command::new(tool)
    };

    command.arg("-delay").arg("10").arg("-loop").arg("0");
    for frame in frames {
        command.arg(frame);
    }
    command.arg(output);

    match command.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("{tool} exited with status {status}")),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Err(format!("{tool} not found")),
        Err(err) => Err(format!("{tool} failed: {err}")),
    }
}

fn create_gif(frames_dir: &Path) -> Result<(), String> {
    let frames = collect_frames(frames_dir)?;
    let output = frames_dir.join("animation.gif");
    let tools = ["magick", "convert"];
    let mut last_err = None;

    for tool in tools {
        match attempt_convert(tool, &frames, &output) {
            Ok(()) => {
                println!("Wrote {}", output.display());
                return Ok(());
            }
            Err(err) => last_err = Some(err),
        }
    }

    Err(last_err.unwrap_or_else(|| "no conversion tool available".to_string()))
}

fn main() {
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let mut step = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell != '@' {
                    continue;
                }

                let neighbors = count_neighbors(&grid, row_idx, col_idx, '@');

                if neighbors < 4 {
                    count += 1;
                    to_remove.push((row_idx, col_idx));
                }
            }
        }

        visualize_step(step, &grid, &to_remove).expect("failed to write visualization");

        if to_remove.is_empty() {
            break;
        }

        remove_rolls(&mut grid, &to_remove);
        step += 1;
    }

    if let Err(err) = create_gif(Path::new("steps")) {
        eprintln!("Warning: failed to create GIF: {}", err);
    }


    println!("Count of cells with fewer than 4 '@' neighbors: {}", count);
}