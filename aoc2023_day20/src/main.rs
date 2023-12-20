use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

struct Message {
    sender: String,
    receiver: String,
    pulse: Pulse,
}

trait Module {
    fn handle_message(&mut self, message: Message) -> Vec<Message>;
}

struct FlipFlop {
    state: bool,
    name: String,
    outputs: Vec<String>,
}

impl Module for FlipFlop {
    fn handle_message(&mut self, message: Message) -> Vec<Message> {
        if message.pulse == Pulse::High {
            Vec::new()
        } else {
            self.state = !self.state;
            let pulse = match self.state {
                true => Pulse::High,
                false => Pulse::Low,
            };

            self.outputs
                .iter()
                .map(|x| Message {
                    receiver: x.to_owned(),
                    sender: self.name.to_owned(),
                    pulse,
                })
                .collect()
        }
    }
}

impl FlipFlop {
    fn new(name: String, outputs: Vec<String>) -> Self {
        Self {
            state: false,
            name,
            outputs,
        }
    }
}

struct Conjunction {
    state: HashMap<String, Pulse>,
    name: String,
    outputs: Vec<String>,
}

impl Module for Conjunction {
    fn handle_message(&mut self, message: Message) -> Vec<Message> {
        self.state.insert(message.sender, message.pulse);

        let pulse = if self.state.values().all(|x| *x == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        self.outputs
            .iter()
            .map(|x| Message {
                sender: self.name.to_owned(),
                receiver: x.to_owned(),
                pulse,
            })
            .collect()
    }
}

impl Conjunction {
    fn new(name: String, outputs: Vec<String>) -> Self {
        let state = HashMap::new();

        Self {
            state,
            name,
            outputs,
        }
    }
}

struct Broadcaster {
    name: String,
    outputs: Vec<String>,
}

impl Module for Broadcaster {
    fn handle_message(&mut self, message: Message) -> Vec<Message> {
        self.outputs
            .iter()
            .map(|receiver| Message {
                sender: self.name.to_owned(),
                receiver: receiver.to_owned(),
                pulse: message.pulse,
            })
            .collect()
    }
}

impl Broadcaster {
    fn new(name: String, outputs: Vec<String>) -> Self {
        Self { name, outputs }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/day20/input").unwrap();

    part_1(parse_modules(&input));
    part_2(parse_modules(&input));
}

fn part_2(mut modules: HashMap<String, Box<dyn Module>>) {
    let mut messages = VecDeque::new();
    let mut vf_highs: HashMap<String, (i32, Option<i32>)> = HashMap::new();

    'outer: for i in 0..10000000 {
        let broadcast_res = modules
            .get_mut("broadcaster")
            .unwrap()
            .handle_message(Message {
                sender: "button".to_owned(),
                receiver: "broadcaster".to_owned(),
                pulse: Pulse::Low,
            });

        for message in broadcast_res {
            messages.push_back(message);
        }

        while let Some(message) = messages.pop_front() {
            if message.receiver == "vf" && message.pulse == Pulse::High {
                let vf = vf_highs.get_mut(&message.sender);
                if let Some((_, second_index)) = vf {
                    if second_index.is_none() {
                        *second_index = Some(i);
                    }
                } else {
                    vf_highs.insert(message.sender.to_owned(), (i, None));
                }

                if vf_highs.len() == 4 && vf_highs.values().all(|(_, second)| second.is_some()) {
                    break 'outer;
                }

                continue;
            }

            let receiver = modules.get_mut(&message.receiver);

            let receiver = match receiver {
                Some(x) => x,
                None => continue,
            };

            let msg = receiver.handle_message(message);

            for message in msg {
                messages.push_back(message);
            }
        }

        //println!()
    }

    let mut cycles = Vec::new();
    for (first, second) in vf_highs.values() {
        let cycle_len = second.unwrap() - first;
        cycles.push(cycle_len as u64);
    }

    let res = cycles.iter().product::<u64>();

    println!("Part 2: {res}");
}

fn part_1(mut modules: HashMap<String, Box<dyn Module>>) {
    let mut messages = VecDeque::new();

    let mut lows = 0;
    let mut highs = 0;

    for _ in 0..1000 {
        let broadcast_res = modules
            .get_mut("broadcaster")
            .unwrap()
            .handle_message(Message {
                sender: "button".to_owned(),
                receiver: "broadcaster".to_owned(),
                pulse: Pulse::Low,
            });

        for message in broadcast_res {
            messages.push_back(message);
        }

        lows += 1;

        while let Some(message) = messages.pop_front() {
            match message.pulse {
                Pulse::High => highs += 1,
                Pulse::Low => lows += 1,
            }

            let receiver = modules.get_mut(&message.receiver);

            let receiver = match receiver {
                Some(x) => x,
                None => continue,
            };

            let msg = receiver.handle_message(message);

            for message in msg {
                messages.push_back(message);
            }
        }
    }

    println!("Done: High {highs} low {lows}, total: {}", highs * lows);
}

fn parse_modules(input: &str) -> HashMap<String, Box<dyn Module>> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    let mut flip_flops = Vec::new();
    let mut broadcast = None;
    let mut conjunctions = Vec::new();

    for line in input.lines() {
        let mut s = line.split(" -> ");
        let type_name = s.next().unwrap();
        let outputs = s.next().unwrap();

        let outputs: Vec<String> = outputs.split(", ").map(|x| x.to_owned()).collect();

        if type_name.starts_with("broadcaster") {
            broadcast = Some(Broadcaster::new("broadcaster".to_string(), outputs));
        } else if let Some(name) = type_name.strip_prefix('%') {
            flip_flops.push(FlipFlop::new(name.to_owned(), outputs));
        } else if let Some(name) = type_name.strip_prefix('&') {
            conjunctions.push(Conjunction::new(name.to_owned(), outputs));
        }
    }

    let broadcast = broadcast.unwrap();

    for i in 0..conjunctions.len() {
        let con_name = conjunctions[i].name.to_owned();
        let mut inputs = Vec::new();
        if broadcast.outputs.contains(&con_name) {
            inputs.push(broadcast.name.to_owned());
        }

        let mut ff = flip_flops
            .iter()
            .filter(|x| x.outputs.contains(&con_name))
            .map(|ff| ff.name.to_owned())
            .collect();

        inputs.append(&mut ff);

        let mut conj = conjunctions
            .iter()
            .filter(|x| x.outputs.contains(&con_name))
            .map(|ff| ff.name.to_owned())
            .collect();

        inputs.append(&mut conj);

        let mut hm = HashMap::new();

        for input in inputs {
            hm.insert(input, Pulse::Low);
        }

        conjunctions[i].state = hm;
    }

    modules.insert("broadcaster".to_owned(), Box::new(broadcast));

    for ff in flip_flops {
        modules.insert(ff.name.to_owned(), Box::new(ff));
    }

    for con in conjunctions {
        modules.insert(con.name.to_owned(), Box::new(con));
    }
    modules
}
