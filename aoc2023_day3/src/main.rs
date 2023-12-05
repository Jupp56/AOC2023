use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut symbol_positions: Vec<(usize, usize)> = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        for (char_num, c) in line.chars().enumerate() {
            match c {
                '.' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => (),
                _ => {
                    symbol_positions.push((line_num, char_num));
                }
            }
        }
    }

    let mut result = 0;

    let lines: Vec<&str> = input.lines().collect();

    let mut possible_gears_with_numbers: HashMap<(usize, usize), Vec<u16>> = HashMap::new();

    for line_num in 0..lines.len() {
        let mut current_number = String::new();
        for (char_num, c) in lines[line_num].chars().enumerate() {
            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    current_number.push(c);
                }
                _ => {
                    if !current_number.is_empty() {
                        //println!("{current_number}");
                        let number: u16 = str::parse(&current_number).unwrap();

                        if add_if_exists_and_char(
                            &mut result,
                            &lines,
                            line_num,
                            char_num,
                            number,
                            &mut possible_gears_with_numbers,
                        ) {
                            current_number.clear();
                            continue;
                        }

                        if char_num > current_number.len() {
                            if add_if_exists_and_char(
                                &mut result,
                                &lines,
                                line_num,
                                char_num - current_number.len() - 1,
                                number,
                                &mut possible_gears_with_numbers,
                            ) {
                                current_number.clear();
                                continue;
                            }
                        }

                        let lower_bound = if char_num > current_number.len() {
                            char_num - current_number.len() - 1
                        } else {
                            char_num - current_number.len()
                        };

                        if line_num > 0 {
                            for i in lower_bound..=char_num {
                                if add_if_exists_and_char(
                                    &mut result,
                                    &lines,
                                    line_num - 1,
                                    i,
                                    number,
                                    &mut possible_gears_with_numbers,
                                ) {
                                    current_number.clear();
                                    continue;
                                }
                            }
                        }

                        if line_num < lines.len() - 1 {
                            for i in lower_bound..=char_num {
                                if add_if_exists_and_char(
                                    &mut result,
                                    &lines,
                                    line_num + 1,
                                    i,
                                    number,
                                    &mut possible_gears_with_numbers,
                                ) {
                                    current_number.clear();
                                    continue;
                                }
                            }
                        }

                        current_number.clear();
                    }
                }
            }
        }

        let char_num = lines[line_num].len() - 1;
        if !current_number.is_empty() {
            //println!("{current_number}");
            let number: u16 = str::parse(&current_number).unwrap();

            if add_if_exists_and_char(
                &mut result,
                &lines,
                line_num,
                char_num - current_number.len(),
                number,
                &mut possible_gears_with_numbers,
            ) {
                current_number.clear();
                continue;
            }

            let lower_bound = char_num - current_number.len();

            if line_num > 0 {
                for i in lower_bound..=char_num {
                    if add_if_exists_and_char(
                        &mut result,
                        &lines,
                        line_num - 1,
                        i,
                        number,
                        &mut possible_gears_with_numbers,
                    ) {
                        current_number.clear();
                        continue;
                    }
                }
            }

            if line_num < lines.len() - 1 {
                for i in lower_bound..=char_num {
                    if add_if_exists_and_char(
                        &mut result,
                        &lines,
                        line_num + 1,
                        i,
                        number,
                        &mut possible_gears_with_numbers,
                    ) {
                        current_number.clear();
                        continue;
                    }
                }
            }

            current_number.clear();
        }
    }

    let mut sum_of_gear_ratios = 0;

    for entry in possible_gears_with_numbers.values() {
        if entry.len() != 2 {
            continue;
        }

        let gear_ratio = entry[0] as u32 * entry[1] as u32;

        sum_of_gear_ratios += gear_ratio;
    }

    for i in 0..10 {
        println!("{:?}", possible_gears_with_numbers.values().collect::<Vec<_>>()[i]);
    }

    println!("Result: {result}. Sum of ratios: {sum_of_gear_ratios}");
}

fn add_if_exists_and_char(
    result: &mut u32,
    lines: &Vec<&str>,
    line_no: usize,
    position: usize,
    current_number: u16,
    possible_gears_with_numbers: &mut HashMap<(usize, usize), Vec<u16>>,
) -> bool {
    match lines[line_no].chars().nth(position) {
        Some(symbol) => {
            if !symbol.is_numeric() && symbol != '.' {
                *result += current_number as u32;

                if symbol == '*' {
                    let v = possible_gears_with_numbers.get_mut(&(line_no, position));
                    match v {
                        None => {
                            possible_gears_with_numbers
                                .insert((line_no, position), vec![current_number]);
                        }
                        Some(v) => {
                            v.push(current_number);
                        }
                    }
                }

                return true;
            }
        }
        None => (),
    }
    false
}

// 554003
