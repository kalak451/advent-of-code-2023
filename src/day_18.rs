#[cfg(test)]
mod day18 {
    use itertools::Itertools;
    use num::abs;
    use regex::Regex;

    use crate::grid::DIR;
    use crate::grid::DIR::{EAST, NORTH, SOUTH, WEST};
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    struct Instruction {
        dir: DIR,
        dist: i64,
    }

    fn parse_instruction_p1(input: &str) -> Instruction {
        let reg = Regex::new(r"([UDLR]) (\d*)").unwrap();
        let caps = reg.captures(input).unwrap();

        return Instruction {
            dir: match caps[1].to_owned().as_str() {
                "U" => NORTH,
                "D" => SOUTH,
                "L" => EAST,
                "R" => WEST,
                _ => panic!("Can't happen")
            },
            dist: caps[2].parse::<i64>().unwrap(),
        };
    }

    fn parse_instruction_p2(input: &str) -> Instruction {
        let reg = Regex::new(r"([UDLR]) (\d*) \(#(.*)\)").unwrap();
        let caps = reg.captures(input).unwrap();

        let hex = &caps[3];

        return Instruction {
            dir: match &hex[5..] {
                "3" => NORTH,
                "1" => SOUTH,
                "2" => EAST,
                "0" => WEST,
                _ => panic!("Can't happen")
            },
            dist: i64::from_str_radix(&hex[0..5], 16).unwrap()
        };
    }

    fn mv(t: (i64, i64), dist: i64, dir: DIR) -> (i64, i64) {
        match dir {
            NORTH => (t.0, t.1 - dist),
            SOUTH => (t.0, t.1 + dist),
            EAST => (t.0 + dist, t.1),
            WEST => (t.0 - dist, t.1),
        }
    }

    fn apply_p1(input: &str) -> i64 {
        let instructions = input.lines().map(|l| parse_instruction_p1(l)).collect_vec();

        return calculate_area(instructions);
    }

    fn apply_p2(input: &str) -> i64 {
        let instructions = input.lines().map(|l| parse_instruction_p2(l)).collect_vec();

        return calculate_area(instructions);
    }

    fn calculate_area(instructions: Vec<Instruction>) -> i64 {
        let mut start = (0, 0);
        let mut data = vec![start.to_owned()];

        for inst in instructions {
            let end = mv(start, inst.dist, inst.dir);
            data.push(end);
            start = end;
        }

        let shoelace_area = calculate_shoelace_area(&data);
        let perimeter = calculate_perimeter(&data);
        return shoelace_area + perimeter / 2 + 1;
    }

    fn calculate_perimeter(data: &Vec<(i64, i64)>) -> i64 {
        data
            .iter()
            .tuple_windows()
            .map(|((x1, y1), (x2, y2))| {
                abs(y2-y1) + abs (x2-x1)
            })
            .sum()
    }

    fn calculate_shoelace_area(data: &Vec<(i64, i64)>) -> i64 {
        let two_a: i64 = data
            .iter()
            .tuple_windows()
            .map(|((x1, y1), (x2, y2))| {
                (x1 * y2) - (y1 * x2)
            })
            .sum();

        abs(two_a / 2)
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
    }

    #[test]
    fn part_1() {
        let data = read_data_file(18, "input.txt");


        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(18, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}