/* fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let lines = input.lines();

    let mut num_games = 0;
    let mut sum_ids = 0;

    'game_loop: for line in lines {
        let game_no_content: Vec<&str> = line.split(':').collect();
        let game_id = str::parse::<u32>(&game_no_content[0][5..]).unwrap();

        let game_iterations = game_no_content[1].split(";");

        for game_iteration in game_iterations {
            let draws = game_iteration.split(",");
            for draw in draws {
                let word_len = if draw.ends_with("red") {
                    4
                } else if draw.ends_with("blue") {
                    5
                } else if draw.ends_with("green") {
                    6
                } else {
                    unreachable!();
                };

                let draw = draw.trim();
                let num = &draw[0..draw.len() - word_len];
                let num = str::parse::<u32>(num).unwrap();

                if word_len == 4 && num > 12 {
                    continue 'game_loop;
                }
                if word_len == 5 && num > 14 {
                    continue 'game_loop;
                }
                if word_len == 6 && num > 13 {
                    continue 'game_loop;
                }
            }
        }

        num_games += 1;
        sum_ids += game_id;
    }

    println!("Number of possible games: {num_games}. ID sum: {sum_ids}");
}
 */

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let lines = input.lines();

    let mut sum_powers = 0;

    for line in lines {
        let game_no_content: Vec<&str> = line.split(':').collect();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        let game_iterations = game_no_content[1].split(";");

        for game_iteration in game_iterations {
            let draws = game_iteration.split(",");
            for draw in draws {
                let word_len = if draw.ends_with("red") {
                    4
                } else if draw.ends_with("blue") {
                    5
                } else if draw.ends_with("green") {
                    6
                } else {
                    unreachable!();
                };

                let draw = draw.trim();
                let num = &draw[0..draw.len() - word_len];
                let num = str::parse::<u32>(num).unwrap();

                if word_len == 4 {
                    min_red = u32::max(min_red, num);
                }
                if word_len == 5 {
                    min_blue = u32::max(min_blue, num);
                }
                if word_len == 6 {
                    min_green = u32::max(min_green, num);
                }
            }
        }
        let power = min_red * min_blue * min_green;

        sum_powers += power;
    }

    println!("Power sum: {sum_powers}");
}
