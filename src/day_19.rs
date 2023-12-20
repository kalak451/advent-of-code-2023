#[cfg(test)]
mod day18 {
    use std::cmp;
    use std::collections::HashMap;

    use itertools::Itertools;
    use regex::Regex;

    use crate::read_data_file;

    static SAMPLE_1: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}"#;

    struct Rule {
        att: Option<String>,
        op: Option<String>,
        value: Option<i64>,
        dest: String,
    }

    impl Rule {
        fn matches(&self, p: &Part) -> bool {
            if self.att.is_none() {
                return true;
            }

            let att = self.att.as_ref().unwrap().as_str();
            let op = self.op.as_ref().unwrap().as_str();
            let value = self.value.unwrap();

            match op {
                "<" => {
                    match att {
                        "x" => p.x < value,
                        "m" => p.m < value,
                        "a" => p.a < value,
                        "s" => p.s < value,
                        _ => panic!("Cant get here")
                    }
                }
                ">" => {
                    match att {
                        "x" => p.x > value,
                        "m" => p.m > value,
                        "a" => p.a > value,
                        "s" => p.s > value,
                        _ => panic!("Cant get here")
                    }
                }
                _ => panic!("can't get here either")
            }
        }
    }

    struct Part {
        x: i64,
        m: i64,
        a: i64,
        s: i64,
    }

    #[derive(Clone)]
    struct PartRange {
        x_min: i64,
        x_max: i64,
        m_min: i64,
        m_max: i64,
        a_min: i64,
        a_max: i64,
        s_min: i64,
        s_max: i64,
    }


    impl PartRange {
        fn new() -> PartRange {
            return PartRange {
                x_min: 1,
                x_max: 4000,
                m_min: 1,
                m_max: 4000,
                a_min: 1,
                a_max: 4000,
                s_min: 1,
                s_max: 4000,
            };
        }
    }

    fn parse_data(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
        let (rules_str, parts_str) = input.split("\n\n").collect_tuple().unwrap();

        let rules = parse_workflows(rules_str);
        let parts = parse_parts(parts_str);

        return (rules, parts);
    }

    fn parse_parts(input: &str) -> Vec<Part> {
        let regx = Regex::new(r"\{x=(\d*),m=(\d*),a=(\d*),s=(\d*)}").unwrap();
        input
            .lines()
            .map(|l| regx.captures(l).unwrap())
            .map(|c| Part {
                x: (&c[1]).parse::<i64>().unwrap(),
                m: (&c[2]).parse::<i64>().unwrap(),
                a: (&c[3]).parse::<i64>().unwrap(),
                s: (&c[4]).parse::<i64>().unwrap(),
            })
            .collect_vec()
    }

    fn parse_workflows(input: &str) -> HashMap<String, Vec<Rule>> {
        let name_regex = Regex::new(r"(.*)\{(.*)}").unwrap();
        return input
            .lines()
            .map(|l| name_regex.captures(l).unwrap())
            .map(|c| {
                let name = c[1].to_owned();
                let rule_str = &c[2];
                let rules = parse_rules(rule_str);

                return (name, rules);
            })
            .collect::<HashMap<_, _>>();
    }

    fn parse_rules(input: &str) -> Vec<Rule> {
        input
            .split(",")
            .map(|s| parse_rule(s))
            .collect_vec()
    }

    fn parse_rule(input: &str) -> Rule {
        if !input.contains(':') {
            return Rule {
                att: None,
                op: None,
                value: None,
                dest: input.to_owned(),
            };
        }

        let regex = Regex::new(r"(.*)([<>])(\d*):(.*)").unwrap();
        let caps = regex.captures(input).unwrap();

        return Rule {
            att: Some(caps[1].to_owned()),
            op: Some(caps[2].to_owned()),
            value: Some(caps[3].parse::<i64>().unwrap()),
            dest: caps[4].to_owned(),
        };
    }

    fn apply_p2(input: &str) -> i64 {
        let (workflows, _) = parse_data(input);

        let (approved, rejected) = walk_tree(PartRange::new(), workflows.get("in").unwrap(), &workflows);

        approved
            .iter()
            .map(|pr| {
                (pr.x_max - pr.x_min + 1) * (pr.m_max - pr.m_min + 1) * (pr.a_max - pr.a_min + 1) * (pr.s_max - pr.s_min + 1)
            })
            .sum()
    }

    fn apply_op_to_range(op: &str, val: i64, rng: (i64, i64)) -> (i64, i64) {
        match op {
            ">" => (cmp::max(rng.0, val + 1), rng.1),
            "<" => (rng.0, cmp::min(rng.1, val - 1)),
            ">=" => (cmp::max(rng.0, val), rng.1),
            "<=" => (rng.0, cmp::min(rng.1, val)),
            _ => panic!("Err")
        }
    }

    fn apply_rule_to_part_range(att: &str, op: &str, value: i64, part_range: &PartRange) -> PartRange {
        let mut new_range = part_range.clone();
        match att {
            "x" => {
                (new_range.x_min, new_range.x_max) = apply_op_to_range(op, value, (new_range.x_min, new_range.x_max))
            }
            "m" => {
                (new_range.m_min, new_range.m_max) = apply_op_to_range(op, value, (new_range.m_min, new_range.m_max))
            }
            "a" => {
                (new_range.a_min, new_range.a_max) = apply_op_to_range(op, value, (new_range.a_min, new_range.a_max))
            }
            "s" => {
                (new_range.s_min, new_range.s_max) = apply_op_to_range(op, value, (new_range.s_min, new_range.s_max))
            }
            _ => panic!("Err")
        }

        return new_range;
    }

    fn walk_tree(pr: PartRange, wf: &Vec<Rule>, workflows: &HashMap<String, Vec<Rule>>) -> (Vec<PartRange>, Vec<PartRange>) {
        let mut approved = vec![];
        let mut rejected = vec![];

        let mut remaining_range = pr.clone();
        for r in wf {
            let mut passed_range = remaining_range.clone();

            if r.att.is_some() {
                let att = r.att.as_ref().unwrap().as_str();
                let op = r.op.as_ref().unwrap().as_str();
                let value = r.value.unwrap();

                let opposite_op = if op == "<" { ">=" } else { "<=" };
                passed_range = apply_rule_to_part_range(att, op, value, &remaining_range);
                remaining_range = apply_rule_to_part_range(att, opposite_op, value, &remaining_range);
            }


            if r.dest == "A" {
                approved.push(passed_range);
            } else if r.dest == "R" {
                rejected.push(passed_range)
            } else {
                let (mut a, mut r) = walk_tree(passed_range, workflows.get(&r.dest).unwrap(), workflows);

                approved.append(&mut a);
                rejected.append(&mut r);
            }
        }

        return (approved, rejected);
    }

    fn apply_p1(input: &str) -> i64 {
        let (workflows, parts) = parse_data(input);

        let mut accepted = vec![];
        let mut rejected = vec![];


        for p in parts {
            let mut wf = workflows.get("in").unwrap();
            loop {
                let matched_rule = wf
                    .iter()
                    .find_or_first(|r| r.matches(&p))
                    .unwrap();

                match matched_rule.dest.as_str() {
                    "A" => {
                        accepted.push(p);
                        break;
                    }
                    "R" => {
                        rejected.push(p);
                        break;
                    }
                    d => wf = workflows.get(d).unwrap()
                }
            }
        }

        let result = accepted
            .iter()
            .map(|p| p.x + p.m + p.a + p.s)
            .sum();

        return result;
    }

    #[test]
    fn sample_1_p1() {
        let data = SAMPLE_1;


        let results = apply_p1(data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_1_p2() {
        let data = SAMPLE_1;

        let results = apply_p2(data);
        println!("Answer: {results:?}");


        // 167409079868000  // Correct Answer
        // 167409079868000
    }

    #[test]
    fn part_1() {
        let data = read_data_file(19, "input.txt");


        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(19, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}