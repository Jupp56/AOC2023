use std::time::Instant;

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Copy, Clone, Debug)]

struct Conversion {
    source_start: isize,
    source_end: isize,
    offset: isize,
}

impl From<&str> for Conversion {
    fn from(value: &str) -> Self {
        let mut numbers = value.split(' ').map(|x| str::parse::<isize>(x).unwrap());
        let destination_start = numbers.next().unwrap();
        let source_start = numbers.next().unwrap();
        let length = numbers.next().unwrap();
        Self {
            source_start,
            source_end: source_start + length - 1,
            offset: destination_start - source_start,
        }
    }
}

impl Conversion {
    pub fn convert(&self, number: isize) -> isize {
        number + self.offset
    }
}

#[derive(Clone, Debug)]
struct Converter {
    conversions: Vec<Conversion>,
}

impl Converter {
    pub fn convert(&self, number: isize) -> isize {
        self.conversions
            .iter()
            .find(|conversion| conversion.source_start <= number && conversion.source_end >= number)
            .map_or_else(|| number, |conversion| conversion.convert(number))
    }
}

fn main() {
    let s = std::fs::read_to_string("../input/day5/input").unwrap();
    part_1(&s);

    let start = Instant::now();
    part_2(&s);
    let end = Instant::now();

    println!("Time: {}", (end - start).as_secs_f32())
}

fn part_1(s: &str) {
    let mut lines = s.lines();

    let seed_list = lines.next().unwrap();
    let seed_list: Vec<isize> = seed_list
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| str::parse::<isize>(x).unwrap())
        .collect();

    let converters = parse_converters(lines);

    let result: isize = seed_list
        .par_iter()
        .map(|seed| {
            let mut result = *seed;
            for converter in &converters {
                result = converter.convert(result);
            }
            result
        })
        .min()
        .unwrap();

    println!("Result Part 1: {result}");
}

fn part_2(s: &str) {
    let mut lines = s.lines();

    let seed_list = lines.next().unwrap();
    let seed_list: Vec<isize> = seed_list
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| str::parse::<isize>(x).unwrap())
        .collect();

    let mut seed_pairs = Vec::new();

    let mut first = -1;
    for seed in &seed_list {
        if first == -1 {
            first = *seed;
        } else {
            seed_pairs.push((first, *seed));

            first = -1;
        }
    }

    let converters = parse_converters(lines);

    let mut results = Vec::new();
    for (start, len) in seed_pairs {
        let result: isize = (start..start + len)
            .into_par_iter()
            .map(|seed| {
                let mut result = seed;
                for converter in &converters {
                    result = converter.convert(result);
                }
                result
            })
            .min()
            .unwrap();
        results.push(result);
    }

    println!("Result Part 2: {}", results.iter().min().unwrap());
}

fn parse_converters(lines: std::str::Lines<'_>) -> Vec<Converter> {
    let mut converters = Vec::new();
    let mut current_conversion_list = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.contains(':') {
            if !current_conversion_list.is_empty() {
                converters.push(Converter {
                    conversions: current_conversion_list,
                });

                current_conversion_list = Vec::new();
            }
        } else {
            current_conversion_list.push(Conversion::from(line));
        }
    }

    converters.push(Converter {
        conversions: current_conversion_list,
    });
    converters
}
