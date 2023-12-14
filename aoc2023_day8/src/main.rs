use std::collections::HashMap;

type WayMap = HashMap<[char; 3], ([char; 3], [char; 3])>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Movement {
    L,
    R,
}

impl From<char> for Movement {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::R,
            'L' => Self::L,
            _ => panic!("Unknown movement direction found"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ElementMovement {
    element_index: usize,
    movement_index: usize,
}

struct StartOffset {
    start: usize,
    offset: Option<usize>,
}

fn main() {
    let (movement_instructions, ways) = parse_input();

    part_1(&ways, &movement_instructions);
    part_2(ways, movement_instructions);
}

fn part_1(ways: &WayMap, movement_instructions: &Vec<Movement>) {
    let mut current_element = ['A', 'A', 'A'];
    let mut steps = 0;

    'outer: loop {
        for movement in movement_instructions {
            let way = ways.get(&current_element).unwrap();
            let new_element = match movement {
                Movement::L => way.0,
                Movement::R => way.1,
            };

            current_element = new_element;
            steps += 1;
            if current_element == ['Z', 'Z', 'Z'] {
                break 'outer;
            }
        }
    }

    println!("Steps: {steps}");
}

fn part_2(ways: WayMap, movement_instructions: Vec<Movement>) {
    let mut current_elements: Vec<[char; 3]> =
        ways.keys().filter(|x| x[2] == 'A').copied().collect();

    let mut steps = 0;

    // Map from element index and movement index to start and potentially offset
    let mut map: HashMap<ElementMovement, StartOffset> = HashMap::new();

    'outer: loop {
        for (movement_index, movement) in movement_instructions.iter().enumerate() {
            for (element_index, current_element) in current_elements.iter_mut().enumerate() {
                let way = ways.get(current_element).unwrap();
                let new_element = match movement {
                    Movement::L => way.0,
                    Movement::R => way.1,
                };

                let ele_mov = ElementMovement {
                    element_index,
                    movement_index,
                };

                if current_element[2] == 'Z' {
                    match map.get_mut(&ele_mov) {
                        Some(start_offset) => {
                            if start_offset.offset.is_none() {
                                start_offset.offset = Some(steps - start_offset.start)
                            }
                        }
                        None => {
                            map.insert(
                                ele_mov,
                                StartOffset {
                                    start: steps,
                                    offset: None,
                                },
                            );
                        }
                    }
                }

                if !map.is_empty()
                    && map
                        .values()
                        .all(|start_offset| start_offset.offset.is_some())
                {
                    break 'outer;
                }

                *current_element = new_element;
            }
            steps += 1;
        }
    }

    let res = part_2_get_smallest_common_multiple(map);

    println!("res: {res}");
}

fn part_2_get_smallest_common_multiple(
    position_end: HashMap<ElementMovement, StartOffset>,
) -> usize {
    // The result only has one value of each element index at movement index 0.
    let mut positions: Vec<usize> = position_end.values().map(|x| x.offset.unwrap()).collect();

    let biggest_number = *positions.iter().max().unwrap();

    // We do not need to check if the biggest number is dividable by itself
    positions.retain(|x| *x != biggest_number);

    let mut i = 1;
    loop {
        if positions.iter().all(|x| (i * biggest_number) % x == 0) {
            break i * biggest_number;
        }

        i += 1;
    }
}

fn parse_input() -> (Vec<Movement>, WayMap) {
    let input = std::fs::read_to_string("../input/day8/input").unwrap();
    let mut lines = input.lines();
    let movement_instructions: Vec<Movement> =
        lines.next().unwrap().chars().map(Movement::from).collect();

    // empty line
    let _ = lines.next();

    let mut ways: HashMap<[char; 3], ([char; 3], [char; 3])> = HashMap::new();

    for line in lines {
        let start = &line[0..3];

        let dest_left = &line[7..10];
        let dest_right = &line[12..15];

        ways.insert(to_array(start), (to_array(dest_left), to_array(dest_right)));
    }
    (movement_instructions, ways)
}

fn to_array<const N: usize>(s: &str) -> [char; N] {
    let mut chars = s.chars();
    [(); N].map(|_| chars.next().unwrap())
}
