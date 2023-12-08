#[cfg(test)]
mod day07 {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::read_data_file;

    static SAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;


    struct Hand {
        original: String,
        score_type: usize,
        tie_vector: Vec<usize>,
        wager: usize,
    }

    impl PartialEq<Self> for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.score_type.eq(&other.score_type)
                && self.tie_vector.eq(&other.tie_vector)
        }
    }

    impl Eq for Hand {}

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.score_type == other.score_type {
                return self.tie_vector.cmp(&other.tie_vector);
            }

            return self.score_type.cmp(&other.score_type);
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            return Some(self.cmp(&other));
        }
    }

    impl Hand {
        fn from_str(s: &str) -> Hand {
            let (original, wager_str) = s.split(" ").collect_tuple().unwrap();

            return Hand {
                original: original.to_owned(),
                score_type: score_hand(original),
                tie_vector: tie_vector(original),
                wager: wager_str.parse::<usize>().unwrap(),
            };
        }

        fn from_str_p2(s: &str) -> Hand {
            let (original, wager_str) = s.split(" ").collect_tuple().unwrap();

            return Hand {
                original: original.to_owned(),
                score_type: score_hand_p2(original),
                tie_vector: tie_vector_p2(original),
                wager: wager_str.parse::<usize>().unwrap(),
            };
        }
    }

    fn score_hand(hand: &str) -> usize {
        let groups = hand
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                if acc.contains_key(&c) {
                    acc.insert(c, acc[&c] + 1);
                } else {
                    acc.insert(c, 1);
                }

                acc
            });

        let counts = groups.values().sorted().rev().collect_vec();
        let hand_types = build_hand_types_map();

        if counts.len() == 1 && *counts[0] == 5 {
            return hand_types["FIVE"];
        }

        if counts.len() == 2 && *counts[0] == 4 {
            return hand_types["FOUR"];
        }

        if counts.len() == 2 && *counts[0] == 3 {
            return hand_types["FULL"];
        }

        if counts.len() == 3 && *counts[0] == 3 {
            return hand_types["THREE"];
        }

        if counts.len() == 3 && *counts[0] == 2 && *counts[1] == 2 {
            return hand_types["TWO_PAIR"];
        }

        if counts.len() == 4 && *counts[0] == 2 {
            return hand_types["PAIR"];
        }

        return hand_types["HIGH"];
    }

    fn score_hand_p2(hand: &str) -> usize {
        let groups = hand
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                if acc.contains_key(&c) {
                    acc.insert(c, acc[&c] + 1);
                } else {
                    acc.insert(c, 1);
                }

                acc
            });

        let joker_count: usize = if groups.contains_key(&'J') { groups[&'J'] } else { 0usize };

        let counts = groups.into_iter().filter(|(k, v)| *k != 'J').map(|(k, v)| v).sorted().rev().collect_vec();
        let hand_types = build_hand_types_map();

        if joker_count == 5 {
            return hand_types["FIVE"];
        }

        if counts.len() == 1 && (counts[0] + joker_count) == 5 {
            return hand_types["FIVE"];
        }

        if counts.len() == 2 && (counts[0] + joker_count) == 4 {
            return hand_types["FOUR"];
        }

        if counts.len() == 2 {
            return hand_types["FULL"];
        }


        if counts.len() == 3 && (counts[0] + joker_count) == 3 {
            return hand_types["THREE"];
        }

        if counts.len() == 3 {
            return hand_types["TWO_PAIR"];
        }

        if counts.len() == 4 && (counts[0] + joker_count) == 2 {
            return hand_types["PAIR"];
        }

        return hand_types["HIGH"];
    }

    fn build_strength_map() -> HashMap<char, usize> {
        HashMap::from([
            ('2', 1),
            ('3', 2),
            ('4', 3),
            ('5', 4),
            ('6', 5),
            ('7', 6),
            ('8', 7),
            ('9', 8),
            ('T', 9),
            ('J', 10),
            ('Q', 11),
            ('K', 12),
            ('A', 13),
        ])
    }

    fn build_strength_map_p2() -> HashMap<char, usize> {
        HashMap::from([
            ('J', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('Q', 11),
            ('K', 12),
            ('A', 13),
        ])
    }

    fn build_hand_types_map() -> HashMap<String, usize> {
        HashMap::from([
            ("HIGH".to_owned(), 1),
            ("PAIR".to_owned(), 2),
            ("TWO_PAIR".to_owned(), 3),
            ("THREE".to_owned(), 4),
            ("FULL".to_owned(), 5),
            ("FOUR".to_owned(), 6),
            ("FIVE".to_owned(), 7),
        ])
    }

    fn tie_vector(hand: &str) -> Vec<usize> {
        let strength = build_strength_map();
        hand.chars()
            .map(|c| strength[&c])
            .collect_vec()
    }

    fn tie_vector_p2(hand: &str) -> Vec<usize> {
        let strength = build_strength_map_p2();
        hand.chars()
            .map(|c| strength[&c])
            .collect_vec()
    }

    fn parse_input(input: &str) -> Vec<Hand> {
        input
            .lines()
            .map(|l| Hand::from_str(l))
            .collect_vec()
    }

    fn parse_input_p2(input: &str) -> Vec<Hand> {
        input
            .lines()
            .map(|l| Hand::from_str_p2(l))
            .collect_vec()
    }

    #[test]
    fn sample_p1() {
        let data = SAMPLE;
        let hands = parse_input(data).into_iter().sorted().collect_vec();

        let aaa: usize = hands.iter()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.wager)
            .sum1()
            .unwrap();

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn sample_p2() {
        let data = SAMPLE;
        let hands = parse_input_p2(data).into_iter().sorted().collect_vec();

        let aaa: usize = hands.iter()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.wager)
            .sum1()
            .unwrap();

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(7, "input.txt");
        let hands = parse_input(&data).into_iter().sorted().collect_vec();

        let aaa: usize = hands.iter()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.wager)
            .sum1()
            .unwrap();

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(7, "input.txt");
        let hands = parse_input_p2(&data).into_iter().sorted().collect_vec();

        let aaa: usize = hands.iter()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.wager)
            .sum1()
            .unwrap();

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn p2_score_hand_tests() {
        let ht = build_hand_types_map();
        assert_eq!(ht["FULL"], score_hand_p2("J3322"));
        assert_eq!(ht["THREE"], score_hand_p2("J3321"));
        assert_eq!(ht["TWO_PAIR"], score_hand_p2("21321"));
        assert_eq!(ht["FIVE"], score_hand_p2("JJJJJ"));
    }
}