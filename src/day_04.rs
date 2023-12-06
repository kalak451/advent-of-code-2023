#[cfg(test)]
mod day04 {
    use std::collections::{HashSet, VecDeque};
    use itertools::Itertools;
    use regex::Regex;

    use crate::read_data_file;

    static SAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    struct Card {
        card_no: i32,
        // winning_numbers: Vec<i32>,
        // numbers_i_have: Vec<i32>,
        matches: usize
    }

    fn parse_card(line: &str) -> Card {
        let card_regex = Regex::new(r"Card +(\d+): ([ \d]+) \| ([ \d]+)").unwrap();
        let caps = card_regex.captures(line).unwrap();
        let card_no = &caps[1].parse::<i32>().unwrap();

        let winning = num_str_to_numbers(&caps[2]);
        let numbers = num_str_to_numbers(&caps[3]);
        let matches = score(&winning, &numbers);

        return Card {
            card_no: card_no.to_owned(),
            // winning_numbers: winning,
            // numbers_i_have: numbers,
            matches
        };
    }

    fn num_str_to_numbers(num_str: &str) -> Vec<i32> {
        num_str
            .trim()
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>()
    }

    fn score(winning: &Vec<i32>, numbers: &Vec<i32>) -> usize {
        let w = winning.into_iter().collect::<HashSet<&i32>>();
        let h = numbers.into_iter().collect::<HashSet<&i32>>();
        let u = w.intersection(&h).collect_vec();
        return u.len();
    }

    #[test]
    fn sample_p1() {
        let data = SAMPLE;
        let cards = data.lines().map(parse_card).collect::<Vec<_>>();

        let aaa: i32 = cards
            .iter()
            .map(|c| c.matches)
            .filter(|m| m > &(0usize))
            .map(|s| (2i32).pow(s as u32 - 1))
            .sum();

        println!("Answer: {aaa:?}");

        //1 2 4 8 16
    }

    #[test]
    fn part_1() {
        let data = read_data_file(4, "input.txt");
        let cards = data.lines().map(parse_card).collect::<Vec<_>>();
        let aaa: i32 = cards
            .iter()
            .map(|c| c.matches)
            .filter(|m| m > &(0usize))
            .map(|s| (2i32).pow(s as u32 - 1))
            .sum();

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(4, "input.txt");
        // let data = SAMPLE;
        let cards = data.lines().map(parse_card).collect::<Vec<_>>();

        let mut queue: VecDeque<usize> = VecDeque::from((0..cards.len()).collect_vec());
        let mut total = 0i64;

        while !queue.is_empty() {
            total = total + 1;
            let idx = queue.pop_back().unwrap();
            let c = &cards[idx];

            for i in c.card_no .. (c.card_no + c.matches as i32) {
                queue.push_front(i as usize);
            }
        }


        let aaa = total;

        println!("Answer: {aaa:?}");
    }
}