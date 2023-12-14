fn main() {
    let input = std::fs::read_to_string("../input/day13/input").unwrap();

    let puzzles = parse_puzzles(&input);

    let mut rows_mirrored = 0;
    let mut rows_changed_mirrored = 0;
    let mut cols_mirrored = 0;
    let mut cols_changed_mirrored = 0;

    'puzzle_loop: for puzzle in puzzles {
        let mut changed_candidate_lines = false;
        for line in 0..puzzle.len() - 1 {
            if puzzle[line] == puzzle[line + 1] {
                let mut found_different_row = false;
                for j in 0..line {
                    match puzzle.get(line + (line - j) + 1) {
                        None => continue,
                        Some(row) => {
                            if puzzle[j] != *row {
                                found_different_row = true;
                                if changed_candidate_lines {
                                    changed_candidate_lines = false;
                                    break;
                                }
                                if lines_almost_equal(&puzzle[j], row) {
                                    changed_candidate_lines = true;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }

                if !found_different_row {
                    rows_mirrored += line + 1;
                } else if changed_candidate_lines {
                    rows_changed_mirrored += line + 1;
                    break;
                }
            }
        }

        let mut changed_candidate_cols = false;

        for col in 0..puzzle[0].len() - 1 {
            if compare_cols(&puzzle, col, col + 1) {
                let mut found_different_col = false;
                for j in 0..col {
                    let col_to_compare_to = col + (col - j) + 1;
                    match puzzle[0].get(col_to_compare_to) {
                        None => continue,
                        Some(_) => {
                            if !compare_cols(&puzzle, j, col_to_compare_to) {
                                found_different_col = true;
                                if changed_candidate_cols {
                                    changed_candidate_cols = false;
                                    break;
                                }
                                if cols_almost_equal(&puzzle, j, col_to_compare_to) {
                                    changed_candidate_cols = true;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }

                if !found_different_col {
                    cols_mirrored += col + 1;
                } else if changed_candidate_cols {
                    cols_changed_mirrored += col + 1;
                    break;
                }
            }
        }

        // if we by now did not find a second one, try again with changed rows
        if changed_candidate_lines || changed_candidate_cols {
            continue;
        }

        for line in 0..puzzle.len() - 1 {
            let mut found_different_row = false;
            if lines_almost_equal(&puzzle[line], &puzzle[line + 1]) {
                for j in 0..line {
                    match puzzle.get(line + (line - j) + 1) {
                        None => continue,
                        Some(row) => {
                            if puzzle[j] != *row {
                                found_different_row = true;
                                break;
                            }
                        }
                    }
                }

                if !found_different_row {
                    rows_changed_mirrored += line + 1;
                    continue 'puzzle_loop;
                }
            }
        }

        for col in 0..puzzle[0].len() - 1 {
            if cols_almost_equal(&puzzle, col, col + 1) {
                let mut found_different_col = false;
                'cols: for j in 0..col {
                    let col_to_compare_to = col + (col - j) + 1;
                    match puzzle[0].get(col_to_compare_to) {
                        None => continue,
                        Some(_) => {
                            if !compare_cols(&puzzle, j, col_to_compare_to) {
                                found_different_col = true;

                                break 'cols;
                            }
                        }
                    }
                }

                if !found_different_col {
                    cols_changed_mirrored += col + 1;
                    continue 'puzzle_loop;
                }
            }
        }

        panic!("Did not find second mirror!");
    }

    println!(
        " res: {}, res_2 {}",
        rows_mirrored * 100 + cols_mirrored,
        rows_changed_mirrored * 100 + cols_changed_mirrored
    );
}

fn compare_cols(a: &Vec<Vec<char>>, col1: usize, col2: usize) -> bool {
    for line in a {
        if line[col1] != line[col2] {
            return false;
        }
    }

    true
}

fn lines_almost_equal(line_a: &[char], line_b: &[char]) -> bool {
    let mut found_difference = false;
    for i in 0..line_a.len() {
        if line_a[i] == line_b[i] {
            continue;
        }
        if !found_difference {
            found_difference = true;
        } else {
            return false;
        }
    }
    found_difference
}

fn cols_almost_equal(a: &Vec<Vec<char>>, col1: usize, col2: usize) -> bool {
    let mut found_difference = false;
    for line in a {
        if line[col1] != line[col2] {
            if !found_difference {
                found_difference = true;
            } else {
                return false;
            }
        }
    }

    found_difference
}

fn parse_puzzles(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut puzzles = Vec::new();

    let mut current_puzzle = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            if !current_puzzle.is_empty() {
                puzzles.push(current_puzzle);
                current_puzzle = Vec::new();
            }
            continue;
        }

        current_puzzle.push(line.chars().collect());
    }

    if !current_puzzle.is_empty() {
        puzzles.push(current_puzzle);
    }
    puzzles
}
