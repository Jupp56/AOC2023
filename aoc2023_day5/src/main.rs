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
    let s = std::fs::read_to_string("input").unwrap();
    //  part_1(&s);
    let mut start = Instant::now();
    part_2(&s);
    let mut end = Instant::now();
    println!("Time: {}", (end - start).as_secs_f32());
    //part_2(&s);
    //part_2_2(&s);
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

fn part_2_2(s: &str) {
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

    let mut converters = parse_converters(lines);

    for converter in &mut converters {
        converter.generate_missing_conversions();
    }

    //println!("{:#?}", converters[0]);

    let mut min = isize::MAX;
    for (start, len) in seed_pairs {
        let res = convert_recursive(start, start + len, &converters, vec![]);
        min = min.min(res);
    }

    println!("Result part 2_2: {min}");
}

fn convert_recursive(
    range_start: isize,
    range_end: isize,
    converters: &[Converter],
    used_convs: Vec<(isize, isize, Conversion)>,
) -> isize {
    let mut conversions_min_result = isize::MAX;

    for conversion in &converters[0].conversions {
        let conversion_end = conversion.start_start + conversion.length;

        // first: check out if we actually hit the conversion
        // start of conversion is between start and end
        if conversion.start_start >= range_start && conversion.start_start < range_end

        // or end of conversion is between start and end
        || conversion_end >= range_start && conversion_end <= range_end

        // or both start and end of conversion are between start and start + len

        || conversion.start_start >= range_start && conversion_end <= range_end
        // or both or both start and end of conversion are encompassing
        || conversion.start_start <= range_start && conversion_end >= range_end
        {
            // second: find out where exactly we hit it
            // the actual start of converting with this conversion
            let actual_start = isize::max(range_start, conversion.start_start);
            // convert that
            let start_of_range_conversion_result = conversion.convert(actual_start);

            // convert the end of the range
            let converted_end = conversion.convert(conversion_end.min(range_end));

            let mut used_convs = used_convs.clone();

            used_convs.push((range_start, range_end, conversion.to_owned()));

            if converters.len() > 1 {
                // now convert with all further converters
                let result = convert_recursive(
                    start_of_range_conversion_result,
                    converted_end,
                    &converters[1..],
                    used_convs,
                );
                // and update our minimum found result if necessary
                conversions_min_result = conversions_min_result.min(result);
            } else {
                if start_of_range_conversion_result == 92661120 {
                    for (start, end, conv) in &used_convs {
                        if *end < conv.start_start || *start > conv.start_start + conv.length {
                            println!("Somethings up");
                        }
                    }
                    println!("{:#?}", used_convs);
                }
                return start_of_range_conversion_result;
            }
        }
    }

    return conversions_min_result;
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
