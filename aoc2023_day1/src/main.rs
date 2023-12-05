use memchr::memmem;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();

    let res = s
        .lines()
        .map(|x| find_numbers_text_included(x))
        .map(|x| {
            println!("{}, {}", x.0, x.1);
            x
        })
        .map(|(first, second)| combine_numbers(first, second))
        .fold(0_u32, |acc, val| acc + val as u32);

    println!("Result: {}", res);
}

fn find_numbers(s: &str) -> (u8, u8) {
    let mut first: Option<u8> = None;
    let mut second: Option<u8> = None;

    for c in s.chars() {
        if c.is_numeric() {
            if first.is_none() {
                first = Some(c.to_digit(10).unwrap() as u8);
            } else {
                second = Some(c.to_digit(10).unwrap() as u8);
            }
        }
    }

    match (first, second) {
        (None, None) => panic!("Line with no number found"),
        (None, Some(_)) => unreachable!(),
        (Some(first), None) => (first, first),
        (Some(first), Some(second)) => (first, second),
    }
}

fn find_numbers_text_included(haystack: &str) -> (u8, u8) {
    let needles: Vec<(u8, &str)> = vec![
        (1, "1"),
        (2, "2"),
        (3, "3"),
        (4, "4"),
        (5, "5"),
        (6, "6"),
        (7, "7"),
        (8, "8"),
        (9, "9"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];

    let mut first = FirstNumber { n: None };
    let mut second = SecondNumber { n: None };

    for needle in needles {
        let it: Vec<usize> = memmem::find_iter(haystack.as_bytes(), needle.1.as_bytes()).collect();

        if it.len() == 0 {
            continue;
        } else if it.len() == 1 {
            let new_index = it[0];
            first.exchange_if_lower(needle.0, new_index);
            second.exchange_if_higher(needle.0, new_index);
        } else {
            let new_index_1 = it[0];
            first.exchange_if_lower(needle.0, new_index_1);

            let new_index_2 = it.last().unwrap();
            second.exchange_if_higher(needle.0, *new_index_2);
        }
    }

    match (first.n, second.n) {
        (None, None) => panic!("Line with no number found"),
        (None, Some(_)) => unreachable!(),
        (Some(first), None) => (first.value, first.value),
        (Some(first), Some(second)) => (first.value, second.value),
    }
}

fn combine_numbers(first: u8, second: u8) -> u8 {
    first * 10 + second
}

struct NumberInner {
    value: u8,
    index: usize,
}

struct FirstNumber {
    n: Option<NumberInner>,
}

impl FirstNumber {
    fn exchange_if_lower(&mut self, value: u8, index: usize) -> bool {
        match &self.n {
            Some(n) => {
                if n.index > index {
                    self.n = Some(NumberInner { value, index });
                    true
                } else {
                    false
                }
            }
            None => {
                self.n = Some(NumberInner { value, index });
                true
            }
        }
    }
}

struct SecondNumber {
    n: Option<NumberInner>,
}

impl SecondNumber {
    fn exchange_if_higher(&mut self, value: u8, index: usize) -> bool {
        match &self.n {
            Some(n) => {
                if n.index < index {
                    self.n = Some(NumberInner { value, index });
                    true
                } else {
                    false
                }
            }
            None => {
                self.n = Some(NumberInner { value, index });
                true
            }
        }
    }
}
