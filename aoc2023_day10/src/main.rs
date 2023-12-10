use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ground,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::Start,
            _ => panic!("Unknown shape"),
        }
    }
}

impl Pipe {
    fn connects_to(&self, direction: Direction) -> bool {
        match direction {
            Direction::North => {
                *self == Pipe::NorthEast || *self == Pipe::NorthWest || *self == Pipe::Vertical
            }
            Direction::East => {
                *self == Pipe::NorthEast || *self == Pipe::SouthEast || *self == Pipe::Horizontal
            }
            Direction::South => {
                *self == Pipe::SouthEast || *self == Pipe::SouthWest || *self == Pipe::Vertical
            }
            Direction::West => {
                *self == Pipe::NorthWest || *self == Pipe::SouthWest || *self == Pipe::Horizontal
            }
        }
    }

    fn connecting_directions(&self) -> Vec<Direction> {
        match self {
            Pipe::Vertical => vec![Direction::North, Direction::South],
            Pipe::Horizontal => vec![Direction::West, Direction::East],
            Pipe::NorthEast => vec![Direction::North, Direction::East],
            Pipe::NorthWest => vec![Direction::North, Direction::West],
            Pipe::SouthWest => vec![Direction::South, Direction::West],
            Pipe::SouthEast => vec![Direction::South, Direction::East],
            Pipe::Ground => vec![],
            Pipe::Start => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Debug)]
struct Maze {
    inner: Vec<Vec<Pipe>>,
}

impl Maze {
    fn try_go(
        &self,
        line: usize,
        index: usize,
        direction: Direction,
    ) -> Option<(Pipe, usize, usize)> {
        let new_coords = match direction {
            Direction::North => {
                if line > 0 {
                    Some((line - 1, index))
                } else {
                    None
                }
            }
            Direction::East => self
                .inner
                .get(line)
                .and_then(|x| x.get(index + 1))
                .map(|_| (line, index + 1)),
            Direction::South => self
                .inner
                .get(line + 1)
                .and_then(|x| x.get(index))
                .map(|_| (line + 1, index)),
            Direction::West => {
                if index > 0 {
                    Some((line, index - 1))
                } else {
                    None
                }
            }
        };

        if let Some((new_line, new_index)) = new_coords {
            let new_pipe = self.inner[new_line][new_index];
            Some((new_pipe, new_line, new_index))
        } else {
            None
        }
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let map = value
            .lines()
            .map(|line| line.chars().map(|x| Pipe::from(x)).collect::<Vec<Pipe>>())
            .collect();

        Self { inner: map }
    }
}

fn main() {
    //TODO: Find actual start pipe?
    let input = std::fs::read_to_string("input").unwrap();

    let mut maze = Maze::from(input.as_ref());

    let mut start_line = 0;
    let mut start_index = 0;
    'outer: for (line_index, line) in maze.inner.iter().enumerate() {
        for (dir_index, dir) in line.iter().enumerate() {
            if *dir == Pipe::Start {
                start_line = line_index;
                start_index = dir_index;
                break 'outer;
            }
        }
    }

    let start_clone = start_line;
    let index_clone = start_index;
    let mut pipes: HashMap<(usize, usize), Pipe> = HashMap::new();

    let mut len = 0;
    let mut first = true;
    let mut old_direction = Direction::North;

    let mut current_element = Pipe::Start;

    'outer: loop {
        for direction in current_element.connecting_directions() {
            if !first && direction == old_direction.inverse() {
                continue;
            }
            let element = maze.try_go(start_line, start_index, direction);

            match element {
                None => continue,
                Some((pipe, line, index)) => {
                    if first && !pipe.connects_to(direction.inverse()) {
                        continue;
                    }
                    if pipe == Pipe::Start {
                        len += 1;
                        break 'outer;
                    }

                    start_index = index;
                    start_line = line;
                    len += 1;
                    old_direction = direction;
                    first = false;
                    current_element = pipe;

                    pipes.insert((line, index), pipe);
                    continue 'outer;
                }
            }
        }
    }

    // Hacky: just replace the start tile with what it actually is, found out: Manually.
    pipes.insert((start_clone, index_clone), Pipe::NorthWest);
    maze.inner[start_clone][index_clone] = Pipe::NorthWest;

    let mut inners = 0;

    for (line_index, line) in maze.inner.iter().enumerate() {
        // first row cannot contain inside tiles.
        if line_index == 0 {
            continue;
        }
        for (pipe_index, _) in line.iter().enumerate() {
            let mut wall_counter = 0;
            // if part of pipes, continue
            if pipe_index == 0 || pipes.contains_key(&(line_index, pipe_index)) {
                continue;
            }

            // look at each element to the left
            for (left_index, pipe_left) in line[0..pipe_index].iter().enumerate().rev() {
                // if not part of pipes, continue
                if !pipes.contains_key(&(line_index, left_index)) {
                    continue;
                }

                // get pipe type count like the following
                match pipe_left {
                    Pipe::Vertical => {
                        wall_counter += 1;
                    }
                    Pipe::NorthWest => {
                        for (corner_left_index, pipe) in line[0..left_index].iter().enumerate().rev()
                        {
                            if !pipes.contains_key(&(line_index, corner_left_index)) {
                                panic!("Bend found without further bend");
                            }

                            match pipe {
                                Pipe::NorthEast => {
                                    break;
                                }
                                Pipe::SouthEast => {
                                    wall_counter += 1;
                                    break;
                                }
                                Pipe::Start => unimplemented!(),
                                _ => (),
                            }
                        }
                    }
                    Pipe::SouthWest => {
                        for (corner_left_index, pipe) in line[0..left_index].iter().enumerate().rev()
                        {
                            if !pipes.contains_key(&(line_index, corner_left_index)) {
                                panic!("Bend found without further bend");
                            }

                            match pipe {
                                Pipe::SouthEast => {
                                    break;
                                }
                                Pipe::NorthEast => {
                                    wall_counter += 1;
                                    break;
                                }
                                Pipe::Start => unimplemented!(),
                                _ => (),
                            }
                        }
                    }

                    Pipe::Start => unimplemented!(),
                    _ => continue,
                }
                // if vertical, count
                // if horizontal, ignore
                // if north- or south east, ignore (west will be there)
                // if other bend
                // go to the left in a new loop
                //if element not in pipes, panic
                // if bend:
                // if it goes into the same direction (up/down), break;
                // it it goes into the other direction, count wall.
            }

            if wall_counter % 2 == 1 {
                inners += 1;
            }
        }
    }

    println!("Steps to farthest point: {}", len / 2);

    println!("Area within: {} tiles", inners);
}
