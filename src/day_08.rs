#[cfg(test)]
mod day08 {
    use std::collections::HashMap;

    use itertools::Itertools;
    use num::integer::lcm;
    use regex::Regex;

    use crate::read_data_file;

    static SAMPLE_1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
    static SAMPLE_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    static SAMPLE_3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    struct Game {
        directions: Vec<char>,
        node_index: HashMap<String, (String, String)>,
    }

    fn parse_input(input: &str) -> Game {
        let (dir, nodes_str) = input.split("\n\n").collect_tuple().unwrap();

        let rgx = Regex::new(r"^(...) = \((...), (...)\)$").unwrap();

        let node_index = nodes_str
            .lines()
            .map(|l| rgx.captures(l).unwrap())
            .map(|c| ((&c[1]).to_owned(), ((&c[2]).to_owned(), (&c[3]).to_owned())))
            .collect::<HashMap<_, _>>();

        return Game {
            directions: dir.chars().collect_vec(),
            node_index,
        };
    }

    fn apply_p1(game: &Game, start_idx: &str, end_pred: fn(&str) -> bool) -> usize {
        let mut steps = 0usize;
        let mut current_index = start_idx;

        for d in game.directions.iter().cycle() {
            let current_node = game.node_index.get(current_index).unwrap();

            if *d == 'L' {
                current_index = &current_node.0;
            } else {
                current_index = &current_node.1;
            }

            steps = steps + 1;

            if end_pred(&current_index) {
                return steps;
            }
        }

        return 0;
    }

    fn apply_p2(game: &Game) -> usize {
        let starting_pos = game.node_index.keys().filter(|k| k.ends_with('A')).collect_vec();

        return starting_pos
            .iter()
            .map(|sp| apply_p1(game, sp, |x| x.ends_with('Z')))
            .reduce(|a,b| lcm(a,b))
            .unwrap();
    }

    #[test]
    fn sample_p1_1() {
        let data = SAMPLE_1;
        let game = parse_input(data);

        let aaa: usize = apply_p1(&game, "AAA", |x| x == "ZZZ");

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn sample_p1_2() {
        let data = SAMPLE_2;
        let game = parse_input(data);

        let aaa: usize = apply_p1(&game, "AAA", |x| x == "ZZZ");

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn sample_p2_3() {
        let data = SAMPLE_3;
        let game = parse_input(data);

        let aaa: usize = apply_p2(&game);

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(8, "input.txt");
        let game = parse_input(&data);

        let aaa: usize = apply_p1(&game, "AAA", |x| x == "ZZZ");

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(8, "input.txt");
        let game = parse_input(&data);

        let aaa: usize = apply_p2(&game);

        println!("Answer: {aaa:?}");
    }
}