mod part_1;
mod part_2;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    part_1::part_1(&input);
    part_2::part_2(&input);
}

fn get_hands<'a, T: TryFrom<&'a str>>(input: &'a str) -> Vec<(T, usize)>
where
    <T as TryFrom<&'a str>>::Error: core::fmt::Debug,
{
    parse_lines(input)
}

fn parse_lines<'a, T: TryFrom<&'a str>>(s: &'a str) -> Vec<(T, usize)>
where
    <T as TryFrom<&'a str>>::Error: core::fmt::Debug,
{
    let mut v = Vec::new();
    for line in s.lines() {
        let mut parts = line.split(' ');
        let hand = T::try_from(parts.next().unwrap()).unwrap();
        let score = str::parse(parts.next().unwrap()).unwrap();
        v.push((hand, score));
    }

    v
}

fn sum_hands<T>(hands: Vec<(T, usize)>) -> usize {
    let mut sum = 0;

    for (rank, hand) in hands.iter().enumerate() {
        sum += hand.1 * (rank + 1);
    }

    sum
}
