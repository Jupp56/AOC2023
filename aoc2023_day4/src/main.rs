fn main() {
    let file = std::fs::read_to_string("../input/day4/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let mut copy_queue = Vec::with_capacity(lines.len());
    for _ in 0..lines.len() {
        copy_queue.push(1);
    }

    let mut card_count = lines.len();

    for line_no in 0..lines.len() {
        let copies = copy_queue[line_no];
        for _ in 1..=copies {
            let mut split_card_no = lines[line_no].split(":");
            let _ = split_card_no.next();
            let numbers = split_card_no.next().unwrap();

            let mut split_numbers = numbers.split('|');

            let winning_numbers = split_numbers.next().unwrap();
            let winning_numbers = parse_numbers(winning_numbers);
            let my_numbers = split_numbers.next().unwrap();
            let my_numbers = parse_numbers(my_numbers);

            let mut card_index = line_no + 1;
            for my_number in my_numbers {
                if winning_numbers.contains(&my_number) {
                    if copy_queue.len() >= card_index {
                        copy_queue[card_index] += 1;
                        card_index += 1;
                        card_count += 1;
                    }
                }
            }
        }
    }

    println!("{card_count}");
}

fn parse_numbers(numbers: &str) -> Vec<u8> {
    let mut res = Vec::new();

    for entry in numbers.split(' ') {
        if !entry.is_empty() {
            res.push(str::parse(entry).unwrap());
        }
    }

    res
}
