use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("../input/day16/input").unwrap();

    let grid: Vec<Vec<(char, HashSet<(usize, usize)>)>> = input
        .lines()
        .map(|line| line.chars().map(|x| (x, HashSet::new())).collect())
        .collect();

    part_1(&grid);

    part_2(grid);
}

fn part_2(grid: Vec<Vec<(char, HashSet<(usize, usize)>)>>) {
    let grid_len = grid.len();
    let col_len = grid[0].len();

    let mut biggest_sum = 0;
    for line in 0..grid.len() {
        let mut new_grid = grid.clone();
        energize_grid(&mut new_grid, line as isize, -1, line as isize, 0);
        biggest_sum = biggest_sum.max(count_energized_fields(&mut new_grid));

        let mut new_grid = grid.clone();
        energize_grid(
            &mut new_grid,
            line as isize,
            col_len as isize,
            line as isize,
            (col_len - 1) as isize,
        );
        biggest_sum = biggest_sum.max(count_energized_fields(&new_grid));
    }

    for col in 0..grid[0].len() {
        let mut new_grid = grid.clone();
        energize_grid(&mut new_grid, -1, col as isize, 0, col as isize);
        biggest_sum = biggest_sum.max(count_energized_fields(&new_grid));

        let mut new_grid = grid.clone();
        energize_grid(
            &mut new_grid,
            grid_len as isize,
            col as isize,
            (grid_len - 1) as isize,
            col as isize,
        );

        biggest_sum = biggest_sum.max(count_energized_fields(&grid));
    }

    println!("Sum 2: {biggest_sum}");
}

fn part_1(grid: &Vec<Vec<(char, HashSet<(usize, usize)>)>>) {
    let mut grid = grid.clone();
    energize_grid(&mut grid, 0, -1, 0, 0);
    println!("Sum 1: {}", count_energized_fields(&grid));
}

fn count_energized_fields(grid: &Vec<Vec<(char, HashSet<(usize, usize)>)>>) -> i32 {
    let mut sum = 0;

    for line in grid {
        for col in line {
            if !col.1.is_empty() {
                sum += 1;
            }
        }
    }
    sum
}

fn energize_grid(
    grid: &mut Vec<Vec<(char, HashSet<(usize, usize)>)>>,
    prev_start_line: isize,
    prev_start_col: isize,
    start_line: isize,
    start_col: isize,
) {
    let mut last_line: isize = prev_start_line;
    let mut last_col: isize = prev_start_col;
    let mut current_line: isize = start_line;
    let mut current_col: isize = start_col;

    loop {
        // break when outside of boundary
        if current_line < 0
            || current_col < 0
            || current_line as usize >= grid.len()
            || current_col as usize >= grid[0].len()
        {
            break;
        }

        // compute next tile
        let next_tile = match grid[current_line as usize][current_col as usize].0 {
            '.' => [
                Some((
                    current_line + (current_line - last_line),
                    current_col + (current_col - last_col),
                )),
                None,
            ],
            '/' => {
                let coords = if current_line > last_line {
                    (current_line, current_col - 1)
                } else if current_line < last_line {
                    (current_line, current_col + 1)
                } else if current_col < last_col {
                    (current_line + 1, current_col)
                } else {
                    (current_line - 1, current_col)
                };

                [Some(coords), None]
            }
            '\\' => {
                let coords = if current_line > last_line {
                    (current_line, current_col + 1)
                } else if current_line < last_line {
                    (current_line, current_col - 1)
                } else if current_col < last_col {
                    (current_line - 1, current_col)
                } else {
                    (current_line + 1, current_col)
                };

                [Some(coords), None]
            }
            '-' => {
                if current_line == last_line {
                    [
                        Some((current_line, current_col + (current_col - last_col))),
                        None,
                    ]
                } else {
                    [
                        Some((current_line, current_col - 1)),
                        Some((current_line, current_col + 1)),
                    ]
                }
            }
            '|' => {
                if current_col == last_col {
                    [
                        Some((current_line + (current_line - last_line), current_col)),
                        None,
                    ]
                } else {
                    [
                        Some((current_line + 1, current_col)),
                        Some((current_line - 1, current_col)),
                    ]
                }
            }

            _ => unimplemented!(),
        };

        let (next_line, next_col) = next_tile[0].unwrap();

        let mut current_was_energized = true;
        // energize current
        if !grid[current_line as usize][current_col as usize]
            .1
            .contains(&(last_line as usize, last_col as usize))
        {
            current_was_energized = false;
            grid[current_line as usize][current_col as usize]
                .1
                .insert((last_line as usize, last_col as usize));
        }

        // break when next is outside of boundary
        if next_line < 0
            || next_col < 0
            || next_line as usize >= grid.len()
            || next_col as usize >= grid[0].len()
        {
            if let Some((next_line, next_col)) = next_tile[1] {
                energize_grid(grid, current_line, current_col, next_line, next_col);
            }
            break;
        }

        // break when hitting an energized tile and next tile is also energized

        if current_was_energized {
            if grid[next_line as usize][next_col as usize]
                .1
                .contains(&(current_line as usize, current_col as usize))
            {
                if let Some((next_line, next_col)) = next_tile[1] {
                    energize_grid(grid, current_line, current_col, next_line, next_col);
                }
                break;
            }
        }

        if let Some((next_line, next_col)) = next_tile[1] {
            energize_grid(grid, current_line, current_col, next_line, next_col);
        }

        last_line = current_line;
        last_col = current_col;

        current_line = next_line;
        current_col = next_col;
    }
}
