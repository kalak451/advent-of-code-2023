#[cfg(test)]
mod day15 {
    use std::collections::VecDeque;

    use itertools::Itertools;

    use crate::read_data_file;

    static SAMPLE_1: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    fn hash(input: &str) -> i64 {
        input
            .chars()
            .fold(0i64, |acc, c | {
                let cv = c as i64;
                let a = acc + cv;
                let b = a * 17;
                let c = b % 256;

                return c;
            })
    }

    fn apply_p1(input: &str) -> i64 {
        input
            .split(",")
            .map(|s| hash(s))
            .sum()
    }

    fn apply_p2(input: &str) -> usize {
        let mut boxes: Vec<VecDeque<(&str, u8)>> = Vec::with_capacity(256);

        for _ in 0..256 {
            boxes.push(VecDeque::new());
        }

        input
            .split(",")
            .for_each(|s| {
                if s.ends_with("-") {
                    let label = &s[0 ..s.len()-1];
                    let box_id = hash(label);
                    let box_list = &mut boxes[box_id as usize];
                    let pos = box_list.iter().find_position(|t| t.0 == label);
                    if pos.is_some() {
                        box_list.remove(pos.unwrap().0);
                    }
                } else {
                    let (label, value_str) = s.split("=").collect_tuple().unwrap();
                    let box_id = hash(label);
                    let box_list = &mut boxes[box_id as usize];
                    let value = value_str.parse::<u8>().unwrap();
                    let pos = box_list.iter().find_position(|t| t.0 == label);

                    if pos.is_some() {
                        let idx = pos.unwrap().0;
                        box_list[idx] = (label, value);
                    } else {
                        box_list.push_back((label, value));
                    }
                }
            });

        let mut result= 0;
        for i in 0..256 {
            let box_list = &boxes[i];
            for j in 0..box_list.len() {
                let (_, value) = box_list[j];
                let score = (i + 1) * (j + 1) * value as usize;
                result = result + score;
            }
        }

        return result;
    }

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
        assert_eq!(30, hash("rn=1"));
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
        let data = read_data_file(15, "input.txt");


        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(15, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}