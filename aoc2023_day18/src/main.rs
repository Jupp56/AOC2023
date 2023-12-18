use std::collections::HashSet;

fn main() {
    let lines = std::fs::read_to_string("../input/day18/input").unwrap();

    let mut sum = 0;
    let mut entries = HashSet::new();
    let mut current_line = 0;
    let mut current_col = 0;

    for line in lines.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let len: isize = str::parse(split.next().unwrap()).unwrap();

        match direction {
            "R" => {
                for i in current_col + 1..=current_col + len {
                    entries.insert((current_line, i));
                }
                current_col += len;
            }
            "L" => {
                for i in (current_col - len..current_col).rev() {
                    entries.insert((current_line, i));
                }
                current_col -= len;
            }
            "D" => {
                for i in current_line + 1..=current_line + len {
                    entries.insert((i, current_col));
                }
                current_line += len;
            }

            "U" => {
                for i in (current_line - len..current_line).rev() {
                    entries.insert((i, current_col));
                }
                current_line -= len;
            }
            _ => panic!("Unknown direction"),
        }
    }

    let farthest_left = entries.iter().map(|x| x.1).min().unwrap();
    let farthest_right = entries.iter().map(|x| x.1).max().unwrap();
    let farthest_up = entries.iter().map(|x| x.0).min().unwrap();
    let farthest_down = entries.iter().map(|x| x.0).max().unwrap();


    for line in farthest_up..=farthest_down {
        //print!("{:03}", line);
        for col in farthest_left..=farthest_right {
            if entries.contains(&(line, col)) {
                //print!("#");
                sum += 1;
                continue;
            }
            let mut walls = 0;

            let mut wall_before = false;
            let mut wall_above = false;
            let mut wall_below = false;
            for i in (farthest_left..col).rev() {
                if entries.contains(&(line, i)) {
                    if !wall_before {
                        if entries.contains(&(line - 1, i)) {
                            wall_above = true;
                        }

                        if entries.contains(&(line + 1, i)) {
                            wall_below = true;
                        }
                        walls += 1;
                        wall_before = true;
                    }
                } else if wall_before {
                    if wall_above && entries.contains(&(line - 1, i + 1)) {
                        walls += 1;
                    }
                    if wall_below && entries.contains(&(line + 1, i + 1)) {
                        walls += 1;
                    }
                    wall_before = false;
                    wall_above = false;
                    wall_below = false;
                }
            }

            if wall_before {
                if line != 0 && wall_above && entries.contains(&(line - 1, farthest_left)) {
                    walls += 1;
                }
                if wall_below && entries.contains(&(line + 1, farthest_left)) {
                    walls += 1;
                }
            }

            if walls % 2 == 1 {
                //println!("{line}, {col} yes");
                sum += 1;
                //print!("i");
            } else {
                //print!(".");
            }
        }
        //println!()
    }

    println!("{sum}");
}
