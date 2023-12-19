use std::{fs, str::FromStr};

use regex::Regex;

const FIRST_WORKFLOW: &str = "in";

struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
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

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part one: {}", part_one(&input));
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
}
