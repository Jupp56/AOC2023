use std::time::Instant;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = std::fs::read_to_string("../input/day6/input").unwrap();
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();

    part_1(time_line, distance_line);
    let start = Instant::now();
    part_2(time_line, distance_line);
    let end = Instant::now();
    println!("Time for 2: {}", (end - start).as_secs_f32());
}

fn part_1(times: &str, distances: &str) {
    let times = parse_line(times);
    let distances: Vec<u64> = parse_line(distances);

    let mut pairs = Vec::new();
    for i in 0..times.len() {
        pairs.push((times[i], distances[i]));
    }

    let res = pairs
        .iter()
        .map(|x| determine_ways_to_win(*x))
        .reduce(|acc, elem| acc * elem)
        .unwrap();

    println!("Result 2: {res}");
}

fn part_2(times: &str, distances: &str) {
    let time = parse_line_2(times);
    let distance = parse_line_2(distances);

    let res = determine_ways_to_win((time, distance));

    println!("Result: {res}");
}

fn determine_ways_to_win(pair: (u64, u64)) -> u64 {
    let time = pair.0;
    let distance = pair.1;

    let sum: u64 = (1..time)
        .into_iter()
        .map(|charge_time_speed| {
            let my_distance = (time - charge_time_speed) * charge_time_speed;
            if my_distance > distance {
                1
            } else {
                0
            }
        })
        .sum();

    sum
}

fn parse_line(line: &str) -> Vec<u64> {
    line.split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| str::parse::<u64>(x).unwrap())
        .collect()
}

fn parse_line_2(line: &str) -> u64 {
    let number = line.split(':').nth(1).unwrap().trim().replace(" ", "");
    str::parse::<u64>(&number).unwrap()
}
