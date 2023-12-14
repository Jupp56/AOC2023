use std::{collections::btree_map::Keys, time::Instant};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Copy, Clone, Debug)]

struct Conversion {
    start_start: isize,
    destination_start: isize,
    length: isize,
}

impl From<&str> for Conversion {
    fn from(value: &str) -> Self {
        let mut numbers = value.split(' ').map(|x| str::parse::<isize>(x).unwrap());
        Self {
            destination_start: numbers.next().unwrap(),
            start_start: numbers.next().unwrap(),
            length: numbers.next().unwrap(),
        }
    }
}

impl Conversion {
    pub fn convert(&self, number: isize) -> isize {
        number + (self.destination_start - self.start_start)
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
            .find(|conversion| {
                conversion.start_start <= number
                    && conversion.start_start + conversion.length > number
            })
            .map_or_else(|| number, |conversion| conversion.convert(number))
    }

    pub fn sort_conversions(&mut self) {
        self.conversions
            .sort_by(|a, b| a.start_start.cmp(&b.start_start));
    }

    pub fn generate_missing_conversions(&mut self) {
        // find smallest start
        // sort by start, then take first
        self.sort_conversions();
        // generate from 0 to start -1 if 0 isnt listed

        let first = self.conversions[0];

        let mut end_elem = first.start_start + first.length - 1;
        if first.start_start != 0 {
            self.conversions.push(Conversion {
                start_start: 0,
                destination_start: 0,
                length: first.start_start,
            });

            self.sort_conversions();
        }

        loop {
            // go to end and look for n+1
            match self
                .conversions
                .iter()
                .find(|x| x.start_start == end_elem + 1)
            {
                Some(elem) => {
                    // if found, go to end, repeat there
                    end_elem = elem.start_start + elem.length - 1;
                    continue;
                }
                None => (),
            }

            // if not found, find smallest one bigger than current
            // are ordered, so just take first that is bigger
            let smallest = self
                .conversions
                .iter()
                .filter(|x| x.start_start > end_elem)
                .next();

            match smallest {
                Some(elem) => {
                    // we clopy it, so that we can modify the list the conversion is in
                    let elem = elem.clone();
                    // if found, generate range up to that -1
                    self.conversions.push(Conversion {
                        start_start: end_elem + 1,
                        destination_start: end_elem + 1,
                        length: elem.start_start - (end_elem + 1),
                    });

                    self.sort_conversions();

                    end_elem = elem.start_start + elem.length;
                }
                None => {
                    // if not found, generate range up to isize::max and break
                    self.conversions.push(Conversion {
                        start_start: end_elem + 1,
                        destination_start: end_elem + 1,
                        length: isize::MAX - (end_elem + 1),
                    });
                    break;
                }
            }
        }
    }
}

fn main() {
    let s = std::fs::read_to_string("../input/day5/input").unwrap();
      part_1(&s);

    part_2(&s);
   
   
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
                    if result == 136096660 {
                        //println!("Last converter: {:?}, seed: {}", converter, seed)
                    }
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
