fn main() {
    let input = std::fs::read_to_string("../input/day14/input").unwrap();
    let mut lines = parse_input(&input);

    let mut configurations = Vec::new();

    //print_lines(&lines);


    let (num_configs, loop_start) = loop {
        slide_circle(&mut lines);
        //println!();
       // print_lines(&lines);
        if configurations.contains(&lines) {
            break (
                configurations.len(),
                configurations.iter().position(|x| x == &lines).unwrap(),
            );
        }

        configurations.push(lines.clone());
    };

    let length_of_loop = num_configs - loop_start;

    println!("length total: {num_configs}, loop start at {loop_start}, length of loop: {length_of_loop}");

    let position_index_at_billion = (1_000_000_000 - (loop_start + 1)) % length_of_loop;

    println!("billion position: {position_index_at_billion}");

    let lines = configurations[loop_start + position_index_at_billion].clone();

    let weight = weight_on_north_beams(&lines);

    println!("Weight: {weight}");


    for (index,set) in configurations.iter().enumerate() {
       // println!("Index: {index} Weight: {}", weight_on_north_beams(&set));
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    lines
}

fn weight_on_north_beams(lines: &Vec<Vec<char>>) -> usize {
    let mut weight = 0;

    for line in 0..lines.len() {
        let load = lines.len() - line;
        for c in &*lines[line] {
            if *c == 'O' {
                weight += load;
            }
        }
    }
    weight
}

fn slide_circle(lines: &mut Vec<Vec<char>>) {
    slide_north(lines);
    slide_west(lines);
    slide_south(lines);
    slide_east(lines);
}

fn slide_north(lines: &mut Vec<Vec<char>>) {
    let mut anything_changed = false;
    loop {
        for i in 1..lines.len() {
            for j in 0..lines[0].len() {
                if lines[i][j] == 'O' && lines[i - 1][j] == '.' {
                    lines[i][j] = '.';
                    lines[i - 1][j] = 'O';
                    anything_changed = true;
                }
            }
        }
        if !anything_changed {
            return;
        }
        anything_changed = false;
    }
}

fn slide_south(lines: &mut Vec<Vec<char>>) {
    let mut anything_changed = false;
    loop {
        for i in (0..lines.len() - 1).rev() {
            for j in 0..lines[0].len() {
                if lines[i][j] == 'O' && lines[i + 1][j] == '.' {
                    lines[i][j] = '.';
                    lines[i + 1][j] = 'O';
                    anything_changed = true;
                }
            }
        }
        if !anything_changed {
            return;
        }
        anything_changed = false;
    }
}

fn slide_west(lines: &mut Vec<Vec<char>>) {
    let mut anything_changed = false;
    loop {
        for i in 0..lines[0].len() {
            for j in 1..lines.len() {
                if lines[i][j] == 'O' && lines[i][j - 1] == '.' {
                    lines[i][j] = '.';
                    lines[i][j - 1] = 'O';
                    anything_changed = true;
                }
            }
        }
        if !anything_changed {
            return;
        }
        anything_changed = false;
    }
}

fn slide_east(lines: &mut Vec<Vec<char>>) {
    let mut anything_changed = false;
    loop {
        for i in 0..lines[0].len() {
            for j in (0..lines.len() - 1).rev() {
                if lines[i][j] == 'O' && lines[i][j + 1] == '.' {
                    lines[i][j] = '.';
                    lines[i][j + 1] = 'O';
                    anything_changed = true;
                }
            }
        }
        if !anything_changed {
            return;
        }
        anything_changed = false;
    }
}

fn print_lines(lines: &Vec<Vec<char>>) {
    for line in lines {
        for char in line {
            print!("{char}");
        }
        println!();
    }
}
