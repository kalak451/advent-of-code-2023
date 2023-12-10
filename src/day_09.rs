#[cfg(test)]
mod day09 {
    use itertools::Itertools;
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    fn parse_input(input: &str) -> Vec<Vec<i64>> {
        input
            .lines()
            .map(|l| l.split(" "))
            .map(|l| l.map(|n| n.parse::<i64>().unwrap()))
            .map(|l| l.collect_vec())
            .collect_vec()
    }

    fn generate_diff_vec(input: &Vec<i64>) -> Vec<i64> {
        let mut result = Vec::new();

        for i in 1..input.len() {
            let diff = input[i] - input[i-1];
            result.push(diff);
        }

        return result;
    }

    fn generate_pyramid(input: &Vec<i64>) -> Vec<Vec<i64>> {
        let mut result = Vec::new();
        result.push(input.to_owned());

        while !result.last().unwrap().iter().all(|i| *i == 0) {
            let r = generate_diff_vec(&result.last().unwrap());
            result.push(r);
        }

        return result;
    }

    fn amend_pyramid(pyramid: &Vec<Vec<i64>>) -> Vec<i64> {
        let mut result = vec![0i64];

        for i in (0..(pyramid.len()-1)).rev() {
            let p = result.last().unwrap();
            let a = pyramid[i].last().unwrap();

            result.push(*p + *a);
        }

        result.reverse();
        return result;
    }

    fn prepend_pyramid(pyramid: &Vec<Vec<i64>>) -> Vec<i64> {
        let mut result = vec![0i64];

        for i in (0..(pyramid.len()-1)).rev() {
            let p = result.last().unwrap();
            let a = pyramid[i].first().unwrap();

            result.push(*a - *p);
        }

        result.reverse();
        return result;
    }

    fn apply_p1(vecs: Vec<Vec<i64>>) -> i64 {
        vecs.iter()
            .map(|l| generate_pyramid(l))
            .map(|p| amend_pyramid(&p))
            .map(|r| *r.first().unwrap())
            .sum()
    }

    fn apply_p2(vecs: Vec<Vec<i64>>) -> i64 {
        vecs.iter()
            .map(|l| generate_pyramid(l))
            .map(|p| prepend_pyramid(&p))
            .map(|r| *r.first().unwrap())
            .sum()
    }

    #[test]
    fn sample_p1() {
        let data = SAMPLE_1;
        let vecs = parse_input(data);

        let results: i64 = apply_p1(vecs);

        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_p2() {
        let data = SAMPLE_1;
        let vecs = parse_input(data);

        let results = apply_p2(vecs);

        println!("Answer: {results:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(9, "input.txt");
        let vecs = parse_input(&data);

        let results: i64 = apply_p1(vecs);

        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(9, "input.txt");
        let vecs = parse_input(&data);

        let results: i64 = apply_p2(vecs);

        println!("Answer: {results:?}");
    }
}