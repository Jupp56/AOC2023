mod part;
use std::sync::atomic::AtomicU64;

use part::Part;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RuleType {
    Greater,
    Smaller,
}

impl From<char> for RuleType {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Smaller,
            '>' => Self::Greater,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RuleResult {
    Accept,
    Reject,
    Workflow(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rule {
    category: Category,
    rule_type: RuleType,
    value: u16,
}

impl Rule {
    fn applies(&self, part: &Part) -> bool {
        let field = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };

        match self.rule_type {
            RuleType::Greater => field > self.value,
            RuleType::Smaller => field < self.value,
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let mut c = value.chars();
        let category = Category::from(c.next().unwrap());
        let rule_type = RuleType::from(c.next().unwrap());
        let value = str::parse::<u16>(&value[2..]).unwrap();

        Self {
            category,
            rule_type,
            value,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct RuleWithResult {
    rule: Rule,
    result: RuleResult,
}

impl RuleWithResult {
    fn apply(&self, part: &Part) -> Option<RuleResult> {
        if self.rule.applies(part) {
            Some(self.result)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct Workflow {
    rules: Vec<RuleWithResult>,
    otherwise: RuleResult,
}

impl Workflow {
    /// applies this workflow to the part given.
    fn apply(&self, part: &Part) -> RuleResult {
        for rule in &self.rules {
            if let Some(result) = rule.apply(part) {
                return result;
            }
        }

        self.otherwise
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EndResult {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone)]
struct Workflows {
    workflows: Vec<Workflow>,
    start_index: usize,
}

impl From<&[&str]> for Workflows {
    fn from(raw_workflows: &[&str]) -> Self {
        let mut workflows = Vec::new();

        let mut start_index = 0;

        for (index, instruction) in raw_workflows.iter().enumerate() {
            let mut name_inst = instruction.split('{');
            let name = name_inst.next().unwrap().to_owned();
            if name == "in" {
                start_index = index;
            }
            let instructions = name_inst.next().unwrap();
            let instructions = &instructions[0..instructions.len() - 1];

            let mut instructions: Vec<&str> = instructions.split(',').collect();

            let otherwise = instructions.pop().unwrap();
            let otherwise = match otherwise {
                "A" => RuleResult::Accept,
                "R" => RuleResult::Reject,
                _ => {
                    let (ins, _) = raw_workflows
                        .iter()
                        .enumerate()
                        .find(|(_, elem)| elem.starts_with(&format!("{otherwise}{{")))
                        .unwrap();

                    RuleResult::Workflow(ins)
                }
            };

            let mut rules = Vec::new();

            for instruction in &instructions {
                let mut split = instruction.split(':');
                let rule = Rule::from(split.next().unwrap());
                let res = split.next().unwrap();
                let result = match res {
                    "A" => RuleResult::Accept,
                    "R" => RuleResult::Reject,
                    _ => {
                        let (ins, _) = raw_workflows
                            .iter()
                            .enumerate()
                            .find(|(_, elem)| elem.starts_with(&format!("{res}{{")))
                            .unwrap();

                        RuleResult::Workflow(ins)
                    }
                };
                rules.push(RuleWithResult { rule, result })
            }

            workflows.push(Workflow { rules, otherwise });
        }

        Self {
            workflows,
            start_index,
        }
    }
}

impl Workflows {
    fn apply(&self, part: &Part) -> EndResult {
        let mut current = &self.workflows[self.start_index];
        loop {
            let res = current.apply(part);
            match res {
                RuleResult::Accept => return EndResult::Accepted,
                RuleResult::Reject => return EndResult::Rejected,
                RuleResult::Workflow(rule_index) => current = &self.workflows[rule_index],
            }
        }
    }
}

fn get_parts_and_workflows(input: &str) -> (Vec<Part>, Workflows) {
    let mut raw_workflows = Vec::new();
    let mut raw_parts = Vec::new();

    let mut found_empty = false;
    for line in input.lines() {
        if line.is_empty() {
            found_empty = true;
        } else if !found_empty {
            raw_workflows.push(line);
        } else {
            raw_parts.push(line);
        }
    }

    let parts: Vec<Part> = raw_parts.into_iter().map(Part::from).collect();

    let workflows = Workflows::from(raw_workflows.as_ref());
    (parts, workflows)
}

fn main() {
    let input = std::fs::read_to_string("../input/day19/input").unwrap();

    let (parts, workflows) = get_parts_and_workflows(&input);

    let mut sum = 0;

    for part in parts {
        if part.x == 1581 && part.m == 287 {
            println!("Weihnachten");
        }
        let res = workflows.apply(&part);

        if res == EndResult::Accepted {
            sum += part.total_rating();
        }
    }

    println!("Part 1: {sum}");

    let mut sum = AtomicU64::new(0);

    for x in 1..=1 {
        println!("x{x}/4000");
        (1..=4000).par_bridge().for_each(|m| {
            println!("m{m}/4000");
            for a in 1..=4000 {
                for s in 1..=4000 {
                    let part = Part { x, m, a, s };

                    let res = workflows.apply(&part);

                    if res == EndResult::Accepted {
                        sum.fetch_add(part.total_rating(), std::sync::atomic::Ordering::SeqCst);
                    }
                }
            }
        });
    }

    println!("Part 2: {}", sum.get_mut());
}
