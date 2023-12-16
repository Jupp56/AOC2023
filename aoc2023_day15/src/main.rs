fn main() {
    part_one();
    part_2();
}


struct Operation {
    _type: OperationType,
    _box: usize,
    label: String,
}

enum OperationType {
    Remove,
    Insert(usize),
}

impl From<&str> for OperationType {
    fn from(value: &str) -> Self {
        match value.chars().next().unwrap() {
            '=' => Self::Insert(str::parse(&value[1..]).unwrap()),
            '-' => Self::Remove,
            _ => panic!(),
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let (label, rest) = if value.contains('=') {
            (&value[..value.len() - 2], &value[value.len() - 2..])
        } else {
            (&value[..value.len() - 1], &value[value.len() - 1..])
        };

        Self {
            label: label.to_owned(),
            _type: OperationType::from(rest),
            _box: hash(label),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct MyBox {
    lenses: Vec<Lense>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lense {
    label: String,
    focal_len: usize,
}

fn part_2() {
    let read_to_string = std::fs::read_to_string("../input/day15/input").unwrap();
    let file = read_to_string;

    let operations: Vec<Operation> = file.split(',').map(Operation::from).collect();

    let mut boxes = Vec::with_capacity(256);

    for _ in 0..256 {
        boxes.push(MyBox { lenses: Vec::new() });
    }

    run_ops(operations, &mut boxes);

    let sum = calculate_total_len(boxes);

    println!("Focus: {sum}");
}

fn run_ops(operations: Vec<Operation>, boxes: &mut [MyBox]) {
    for op in operations {
        match op._type {
            OperationType::Remove => {
                boxes[op._box].lenses.retain(|x| x.label != op.label);
            }
            OperationType::Insert(focal_len) => {
                let b = &mut boxes[op._box];
                match b.lenses.iter_mut().find(|x| x.label == op.label) {
                    Some(lense) => lense.focal_len = focal_len,
                    None => {
                        b.lenses.push(Lense {
                            label: op.label,
                            focal_len,
                        });
                    }
                }
            }
        }
    }
}

fn calculate_total_len(boxes: Vec<MyBox>) -> usize {
    let mut sum = 0;

    for (box_number, b) in boxes.iter().enumerate() {
        for (num_slot, lens) in b.lenses.iter().enumerate() {
            sum += (box_number + 1) * (num_slot + 1) * lens.focal_len;
        }
    }
    sum
}

fn hash(chars: &str) -> usize {
    let mut current_value = 0;
    for c in chars.chars() {
        if c == '\n' {
            continue;
        }

        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn part_one() {
    let file = std::fs::read_to_string("../input/day15/input").unwrap();

    let res: usize = file.split(',').map(hash).sum();

    println!("res1: {res}");
}

#[test]
fn test_hash_hash() {
    assert_eq!(hash("HASH"), 52);
}
