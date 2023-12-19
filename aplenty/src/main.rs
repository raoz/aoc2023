use regex::Regex;
use std::{fmt::Display, fs, str::FromStr};

const FIRST_WORKFLOW: &str = "in";

struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug, Clone)]
struct OpenRange {
    start: i64,
    end: i64,
}

impl OpenRange {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
    fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    fn count(&self) -> u64 {
        if self.end <= self.start {
            0
        } else {
            (self.end - self.start - 1).try_into().unwrap()
        }
    }
}

impl Display for OpenRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

#[derive(Debug, Clone)]
struct AbstractItem {
    x: OpenRange,
    m: OpenRange,
    a: OpenRange,
    s: OpenRange,
}

impl AbstractItem {
    fn empty() -> Self {
        Self {
            x: OpenRange::empty(),
            m: OpenRange::empty(),
            a: OpenRange::empty(),
            s: OpenRange::empty(),
        }
    }
    fn count(&self) -> u64 {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }
}
impl Display for AbstractItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{x={}, m={}, a={}, s={}}}",
            self.x, self.m, self.a, self.s
        )
    }
}

impl Item {
    fn total_rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }

    fn resolve(&self, workflows: &[Workflow]) -> Outcome {
        let mut current_workflow = workflows
            .iter()
            .find(|workflow| workflow.name == FIRST_WORKFLOW)
            .unwrap();
        while let Outcome::Call(next_workflow_name) = current_workflow.apply(self) {
            current_workflow = workflows
                .iter()
                .find(|workflow| workflow.name == next_workflow_name)
                .unwrap();
        }

        current_workflow.apply(self)
    }
}
impl FromStr for Item {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Item {
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            m: caps.get(2).unwrap().as_str().parse().unwrap(),
            a: caps.get(3).unwrap().as_str().parse().unwrap(),
            s: caps.get(4).unwrap().as_str().parse().unwrap(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Reject,
    Accept,
    Call(String),
}

enum Rule {
    Condition {
        param: char,
        operator: char,
        value: i64,
        outcome: Outcome,
    },
    Unconditional(Outcome),
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\w)([<>])(\d+):(\w+)?").unwrap();
        if let Some(caps) = re.captures(s) {
            let param = caps.get(1).unwrap().as_str().chars().next().unwrap();
            let operator = caps.get(2).unwrap().as_str().chars().next().unwrap();
            let value = caps.get(3).unwrap().as_str().parse().unwrap();
            let outcome = match caps.get(4).unwrap().as_str() {
                "R" => Outcome::Reject,
                "A" => Outcome::Accept,
                x => Outcome::Call(x.to_owned()),
            };
            Ok(Rule::Condition {
                param,
                operator,
                value,
                outcome,
            })
        } else {
            let outcome = match s {
                "R" => Outcome::Reject,
                "A" => Outcome::Accept,
                x => Outcome::Call(x.to_owned()),
            };
            Ok(Rule::Unconditional(outcome))
        }
    }
}

impl Rule {
    fn apply(&self, item: &Item) -> Option<Outcome> {
        match self {
            Rule::Condition {
                param,
                operator,
                value,
                outcome,
            } => {
                let param_value = match param {
                    'x' => item.x,
                    'm' => item.m,
                    'a' => item.a,
                    's' => item.s,
                    _ => panic!("Invalid param {param}"),
                };
                let matches = match operator {
                    '<' => param_value < *value,
                    '>' => param_value > *value,
                    _ => panic!("Invalid operator {operator}"),
                };
                if matches {
                    Some(outcome.clone())
                } else {
                    None
                }
            }
            Rule::Unconditional(outcome) => Some(outcome.clone()),
        }
    }

    fn accepting_items(&self, abstract_item: &AbstractItem, workflows: &[Workflow]) -> u64 {
        match self {
            Rule::Unconditional(Outcome::Accept) => abstract_item.count(),
            Rule::Unconditional(Outcome::Reject) => 0,
            Rule::Unconditional(Outcome::Call(fun)) => workflows
                .iter()
                .find(|workflow| workflow.name == *fun)
                .unwrap()
                .accepting_items(abstract_item, workflows),
            Rule::Condition {
                param,
                operator,
                value,
                outcome,
            } => {
                let mut new_abstract_item = abstract_item.clone();
                let param_range = match param {
                    'x' => &mut new_abstract_item.x,
                    'm' => &mut new_abstract_item.m,
                    'a' => &mut new_abstract_item.a,
                    's' => &mut new_abstract_item.s,
                    _ => panic!("Invalid param {param}"),
                };
                match operator {
                    '<' => param_range.end = param_range.end.min(*value),
                    '>' => param_range.start = param_range.start.max(*value),
                    _ => panic!("Invalid operator {operator}"),
                };
                match outcome {
                    Outcome::Accept => new_abstract_item.count(),
                    Outcome::Reject => 0,
                    Outcome::Call(fun) => workflows
                        .iter()
                        .find(|workflow| workflow.name == *fun)
                        .unwrap()
                        .accepting_items(&new_abstract_item, workflows),
                }
            }
        }
    }

    fn non_matching_items(&self, abstract_item: &AbstractItem) -> AbstractItem {
        match self {
            Rule::Condition {
                param,
                operator,
                value,
                outcome: _,
            } => {
                let mut new_abstract_item = abstract_item.clone();
                let param_range = match param {
                    'x' => &mut new_abstract_item.x,
                    'm' => &mut new_abstract_item.m,
                    'a' => &mut new_abstract_item.a,
                    's' => &mut new_abstract_item.s,
                    _ => panic!("Invalid param {param}"),
                };
                match operator {
                    '<' => param_range.start = param_range.start.max(*value - 1),
                    '>' => param_range.end = param_range.end.min(*value + 1),
                    _ => panic!("Invalid operator {operator}"),
                };
                new_abstract_item
            }
            Rule::Unconditional(_) => AbstractItem::empty(),
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules_str) = s.split_once('{').unwrap();
        let rules = rules_str
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|rule_str| rule_str.parse::<Rule>().unwrap())
            .collect::<Vec<_>>();
        Ok(Workflow {
            name: name.to_owned(),
            rules,
        })
    }
}

impl Workflow {
    fn apply(&self, item: &Item) -> Outcome {
        self.rules.iter().find_map(|rule| rule.apply(item)).unwrap()
    }

    fn accepting_items(&self, abstract_item: &AbstractItem, workflows: &[Workflow]) -> u64 {
        let mut total = 0;
        let mut item = abstract_item.clone();
        for rule in &self.rules {
            total += rule.accepting_items(&item, workflows);
            item = rule.non_matching_items(&item);
            if item.count() == 0 {
                break;
            }
        }
        total
    }
}

fn part_one(input: &str) -> i64 {
    let (workflows_str, items_str) = input.split_once("\n\n").unwrap();
    let workflows = workflows_str
        .lines()
        .map(|line| line.parse::<Workflow>().unwrap())
        .collect::<Vec<_>>();
    let items = items_str
        .lines()
        .map(|line| line.parse::<Item>().unwrap())
        .collect::<Vec<_>>();

    items
        .iter()
        .filter(|item| item.resolve(&workflows) == Outcome::Accept)
        .map(Item::total_rating)
        .sum()
}

fn part_two(input: &str) -> u64 {
    let (workflows_str, _) = input.split_once("\n\n").unwrap();
    let workflows = workflows_str
        .lines()
        .map(|line| line.parse::<Workflow>().unwrap())
        .collect::<Vec<_>>();

    let start_workflow = workflows
        .iter()
        .find(|workflow| workflow.name == FIRST_WORKFLOW)
        .unwrap();

    let abstract_item = AbstractItem {
        x: OpenRange::new(0, 4001),
        m: OpenRange::new(0, 4001),
        a: OpenRange::new(0, 4001),
        s: OpenRange::new(0, 4001),
    };

    start_workflow.accepting_items(&abstract_item, &workflows)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parse_item() {
        let item: Item = "{x=787,m=2655,a=1222,s=2876}".parse().unwrap();
        assert_eq!(item.total_rating(), 7540);
    }

    #[test]
    fn test_rule() {
        let rule: Rule = "x>1000:A".parse().unwrap();
        let item: Item = "{x=787,m=2655,a=1222,s=2876}".parse().unwrap();
        assert_eq!(rule.apply(&item), None);
        let item2: Item = "{x=1679,m=44,a=2067,s=496}".parse().unwrap();
        assert_eq!(rule.apply(&item2), Some(Outcome::Accept));
    }

    #[test]
    fn test_workflow() {
        let workflow = "px{a<2006:qkq,m>2090:A,rfg}".parse::<Workflow>().unwrap();
        let item = "{x=787,m=2655,a=1222,s=2876}".parse::<Item>().unwrap();
        assert_eq!(workflow.apply(&item), Outcome::Call("qkq".to_owned()));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE), 19114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE), 167409079868000);
    }
}
