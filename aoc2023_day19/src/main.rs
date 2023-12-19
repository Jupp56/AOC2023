use std::collections::HashMap;
mod part;
use part::Part;

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

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuleResult {
    Accept,
    Reject,
    Workflow(String),
}

impl From<&str> for RuleResult {
    fn from(value: &str) -> Self {
        if value == "A" {
            return Self::Accept;
        } else if value == "R" {
            return Self::Reject;
        }
        Self::Workflow(value.to_owned())
    }
}

impl RuleResult {
    fn is_workflow_with_name(&self, workflow: &str) -> bool {
        matches!(self, RuleResult::Workflow(x) if x == workflow)
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct RuleWithResult {
    rule: Rule,
    result: RuleResult,
}

impl RuleWithResult {
    fn apply(&self, part: &Part) -> Option<RuleResult> {
        if self.rule.applies(part) {
            Some(self.result.clone())
        } else {
            None
        }
    }
}

impl From<&str> for RuleWithResult {
    fn from(value: &str) -> Self {
        let mut s = value.split(':');
        let rule = Rule::from(s.next().unwrap());
        let result = RuleResult::from(s.next().unwrap());
        Self { rule, result }
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

        self.otherwise.clone()
    }

    /// Finds the rule that references the workflow given.
    fn find_referencing_rule(&self, workflow_name: &str) -> &RuleWithResult {
        self.rules
            .iter()
            .find(|x| x.result.is_workflow_with_name(workflow_name))
            .unwrap()
    }

    /// All the rules before the rule given are subtracted in inverse from the restriction.
    /// In other words, the restriction is modified to fail the first rules up to the rule specified.
    fn exclude_rules_before_from_restriction(
        &self,
        other_rule: &Rule,
        restrictions: &mut Restrictions,
    ) {
        for rule in &self.rules {
            if &rule.rule == other_rule {
                break;
            }

            restrictions.intersect_with_opposite_of_rule(&rule.rule);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EndResult {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone)]
struct Workflows {
    workflows: HashMap<String, Workflow>,
}

impl From<&[&str]> for Workflows {
    fn from(raw_workflows: &[&str]) -> Self {
        let mut workflows = HashMap::new();

        for instruction in raw_workflows {
            let mut name_inst = instruction.split('{');
            let name = name_inst.next().unwrap().to_owned();
            let instructions = name_inst.next().unwrap();
            let instructions = &instructions[0..instructions.len() - 1];

            let mut instructions: Vec<&str> = instructions.split(',').collect();

            let otherwise = instructions.pop().unwrap();
            let otherwise = RuleResult::from(otherwise);

            let rules = instructions.into_iter().map(RuleWithResult::from).collect();

            workflows.insert(name, Workflow { rules, otherwise });
        }

        Self { workflows }
    }
}

impl Workflows {
    fn apply(&self, part: &Part) -> EndResult {
        let mut current = self.workflows.get("in").unwrap();
        loop {
            let res = current.apply(part);
            match res {
                RuleResult::Accept => return EndResult::Accepted,
                RuleResult::Reject => return EndResult::Rejected,
                RuleResult::Workflow(name) => current = self.workflows.get(&name).unwrap(),
            }
        }
    }

    /// Returns all workflows that contain a rule that references the workflow provided.
    fn find_referencing_workflows(&self, workflow_name: &str) -> Vec<(&String, &Workflow)> {
        let referencing_flows: Vec<(&String, &Workflow)> = self
        .workflows
        .iter()
        .filter(|(_, x)| {
            x.rules.iter().any(|rule| {
                if let RuleResult::Workflow(wf_name) = &rule.result {
                    return wf_name == workflow_name;
                }
                false
            })
            || matches!(&x.otherwise, RuleResult::Workflow(wf_name) if wf_name == workflow_name)
        })
        .collect();
        referencing_flows
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Restrictions {
    pub x: Interval,
    pub m: Interval,
    pub a: Interval,
    pub s: Interval,
}

impl Restrictions {
    fn sum(&self) -> u128 {
        // TODO: Distinct?
        self.x.sum() * self.m.sum() * self.a.sum() * self.s.sum()
    }
    fn intersect_with_rule(&mut self, rule: &Rule) {
        let field = match rule.category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        };

        field.intersect(rule);
    }

    fn intersect_with_opposite_of_rule(&mut self, rule: &Rule) {
        let field = match rule.category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        };

        field.intersect_opposite(rule);
    }
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    lower: u16,
    upper: u16,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            lower: 1,
            upper: 4000,
        }
    }
}

impl Interval {
    fn sum(&self) -> u128 {
        if self.upper > self.lower {
            self.upper as u128 - self.lower as u128 + 1
        } else {
            0
        }
    }

    fn intersect(&mut self, rule: &Rule) {
        match rule.rule_type {
            RuleType::Greater => self.lower = self.lower.max(rule.value + 1),
            RuleType::Smaller => self.upper = self.upper.min(rule.value - 1),
        }
    }

    fn intersect_opposite(&mut self, rule: &Rule) {
        match rule.rule_type {
            RuleType::Greater => self.upper = self.upper.min(rule.value),
            RuleType::Smaller => self.lower = self.lower.max(rule.value),
        }
    }
}

/// Recursively searches through the referencing workflows until it finds the "in" workflow. Makes the intervals smaller according to the modalities.
/// Sums up all the branches that occur.
/// A workflow needs to have run before the invocation of this method (old_workflow_name refers to this).
fn sum_referencing_workflows(
    all_workflows: &Workflows,
    old_workflow_name: &str,
    current_workflow_name: &str,
    current_workflow: &Workflow,
    mut restrictions: Restrictions,
) -> u128 {
    if current_workflow
        .otherwise
        .is_workflow_with_name(old_workflow_name)
    {
        for rule in &current_workflow.rules {
            restrictions.intersect_with_opposite_of_rule(&rule.rule);
        }
    } else {
        let referencing_rule = current_workflow.find_referencing_rule(old_workflow_name);

        restrictions.intersect_with_rule(&referencing_rule.rule);

        current_workflow
            .exclude_rules_before_from_restriction(&referencing_rule.rule, &mut restrictions);
    }

    if current_workflow_name == "in" {
        return restrictions.sum();
    }

    let referencing_workflows = all_workflows.find_referencing_workflows(current_workflow_name);

    if referencing_workflows.is_empty() {
        return 0;
    }

    let mut sum = 0;

    for (new_name, new_workflow) in referencing_workflows {
        sum += sum_referencing_workflows(
            all_workflows,
            current_workflow_name,
            new_name,
            new_workflow,
            restrictions,
        );
    }

    sum
}

fn part_1(parts: &[Part], workflows: &Workflows) {
    let mut sum = 0;

    for part in parts {
        let res = workflows.apply(part);

        if res == EndResult::Accepted {
            sum += part.total_rating();
        }
    }

    println!("Part 1: {sum}");
}

fn part_2(workflows: &Workflows) {
    let mut sum: u128 = 0;

    for (outer_name, wf) in &workflows.workflows {
        let referencing_flows: Vec<(&String, &Workflow)> =
            workflows.find_referencing_workflows(outer_name);

        for rule in &wf.rules {
            if rule.result == RuleResult::Accept {
                let mut rest = Restrictions::default();
                rest.intersect_with_rule(&rule.rule);

                wf.exclude_rules_before_from_restriction(&rule.rule, &mut rest);

                for (referencing_name, flow) in &referencing_flows {
                    sum += sum_referencing_workflows(
                        workflows,
                        outer_name,
                        referencing_name,
                        flow,
                        rest,
                    );
                }
            }
        }

        if wf.otherwise == RuleResult::Accept {
            let mut rest = Restrictions::default();

            for rule in &wf.rules {
                rest.intersect_with_opposite_of_rule(&rule.rule);
            }

            for (referencing_name, flow) in referencing_flows {
                sum +=
                    sum_referencing_workflows(workflows, outer_name, referencing_name, flow, rest);
            }
        }
    }

    println!("Part 2: {sum}");
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

    part_1(&parts, &workflows);

    part_2(&workflows);
}
