const SIZE_FACTOR: usize = 2;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let universe: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    print_universe(&universe);

    let mut galaxies = Vec::new();

    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let mut galaxies_clone = galaxies.clone();

    for row in (0..universe.len()).rev() {
        if universe[row].iter().all(|x| *x == '.') {
            for galaxy_index in 0..galaxies.len() {
                if galaxies[galaxy_index].0 > row {
                    galaxies_clone[galaxy_index].0 += SIZE_FACTOR - 1;
                }
            }
        }
    }

    for col in (0..universe[0].len()).rev() {
        if universe.iter().all(|x| x[col] == '.') {
            for galaxy_index in 0..galaxies.len() {
                if galaxies[galaxy_index].1 > col {
                    galaxies_clone[galaxy_index].1 += SIZE_FACTOR - 1;
                }
            }
        }
    }

    let mut sum = 0;

    loop {
        let galaxy = galaxies_clone.pop();
        let galaxy = match galaxy {
            None => break,
            Some(x) => x,
        };

        for other_galaxy in &galaxies_clone {
            sum += compute_distance(&galaxy, other_galaxy);
        }
    }

    println!("Sum: {sum}")
}

fn compute_distance(first: &(usize, usize), second: &(usize, usize)) -> usize {
    first.0.abs_diff(second.0) + first.1.abs_diff(second.1)
}

fn print_universe(galaxy: &Vec<Vec<char>>) {
    for row in galaxy {
        for char in row {
            print!("{char}");
        }

        println!();
    }
}
