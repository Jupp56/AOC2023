use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("test-input").unwrap();

    let mut universe: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    print_universe(&universe);

    let empty_row: Vec<char> = universe[0].iter().map(|_| '.').collect();

    let mut additional_rows = Vec::new();
    let mut additional_columns = Vec::new();

    for row in (0..universe.len()).rev() {
        if universe[row].iter().all(|x| *x == '.') {
            additional_rows.push(row);
            //universe.insert(row, empty_row.clone());
        }
    }

    for col in (0..universe[0].len()).rev() {
        if universe.iter().all(|x| x[col] == '.') {
            for row in universe.iter_mut() {
                additional_columns.push(col);
                //row.insert(col, '.');
            }
        }
    }

    println!("\n\nexpanded:\n");
    //print_universe(&universe);

    let mut galaxies = Vec::new();

    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let mut sum = 0;

    loop {
        let galaxy = galaxies.pop();
        let galaxy = match galaxy {
            None => break,
            Some(x) => x,
        };

        for other_galaxy in &galaxies {
            sum += compute_distance(&galaxy, other_galaxy, &additional_rows, &additional_columns);
        }
    }

    println!("Sum: {sum}")
}

fn compute_distance(
    first: &(usize, usize),
    second: &(usize, usize),
    rows: &Vec<usize>,
    cols: &Vec<usize>,
) -> usize {
    let mut adds = rows
        .iter()
        .filter(|x| first.0 > **x && second.0 < **x || first.0 < **x && second.0 > **x)
        .count();

    let adds_cols = cols
        .iter()
        .filter(|x| first.1 > **x && second.1 < **x || first.1 < **x && second.1 > **x)
        .count();

    first.0.abs_diff(second.0) + adds + first.1.abs_diff(second.1) + adds_cols
}

fn print_universe(galaxy: &Vec<Vec<char>>) {
    for row in galaxy {
        for char in row {
            print!("{char}");
        }

        print!("\n");
    }
}
