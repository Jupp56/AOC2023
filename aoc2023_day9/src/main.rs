use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut acc = 0;

    for line in input.lines() {
        let values: Vec<isize> = line
            .split(' ')
            .map(|x| str::parse::<isize>(x).unwrap())
            .collect();

        let mut histories = Vec::new();

        histories.push(values);

        let mut histories_index = 0;
        loop {
            let mut current_history = Vec::new();
            let mut last_val = None;

            for value in &histories[histories_index] {
                match last_val {
                    None => (),
                    Some(val) => {
                        current_history.push(value - val);
                    }
                }

                last_val = Some(value);
            }

            histories.push(current_history);
            histories_index += 1;
            if histories[histories_index].iter().all(|x| *x == 0) {
                break;
            }
        }

        histories.last_mut().unwrap().push(0);

        let mut i = histories.len() - 2;

        loop {
            let new_val = histories[i].last().unwrap() + histories[i + 1].last().unwrap();
            histories[i].push(new_val);

            if i != 0 {
                i -= 1;
            } else {
                break;
            }
        }

        acc += histories[0].last().unwrap();
    }

    println!("Result Part 1: {acc}");
}

fn part_2(input: &str) {
    let mut acc = 0;

    for line in input.lines() {
        let values: VecDeque<isize> = line
            .split(' ')
            .map(|x| str::parse::<isize>(x).unwrap())
            .collect();

        let mut histories = Vec::new();

        histories.push(values);

        let mut histories_index = 0;
        loop {
            let mut current_history = VecDeque::new();
            let mut last_val = None;

            for value in &histories[histories_index] {
                match last_val {
                    None => (),
                    Some(val) => {
                        current_history.push_back(value - val);
                    }
                }

                last_val = Some(value);
            }

            histories.push(current_history);
            histories_index += 1;
            if histories[histories_index].iter().all(|x| *x == 0) {
                break;
            }
        }

        histories.last_mut().unwrap().push_front(0);

        let mut i = histories.len() - 2;

        loop {
            let new_val = histories[i].front().unwrap() - histories[i + 1].front().unwrap();
            histories[i].push_front(new_val);

            if i != 0 {
                i -= 1;
            } else {
                break;
            }
        }

        acc += histories[0].front().unwrap();
    }

    println!("Result Part 2: {acc}");
}
