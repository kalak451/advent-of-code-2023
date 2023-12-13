#[cfg(test)]
mod day12 {
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::read_data_file;

    static SAMPLE_1: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;


    fn is_complete(data: &str) -> bool {
        return !data.contains("?");
    }

    fn is_correct(data: &str, parity: &Vec<usize>) -> bool {
        let x = data
            .split(".")
            .filter(|it| !it.is_empty())
            .collect_vec();

        if x.len() != parity.len() {
            return false;
        }

        let result = x.iter().zip(parity).all(|(str, size)| str.len() == *size);
        return result;
    }

    fn count_solutions(data: &str, parity: &Vec<usize>) -> usize {
        if is_complete(data) {
            if is_correct(data, parity) {
                return 1;
            } else {
                return 0;
            }
        }

        let x = count_solutions(&data.replacen("?", ".", 1), parity);
        let y = count_solutions(&data.replacen("?", "#", 1), parity);

        return x + y;
    }

    fn count_solutions_better(data: &str, runs: &[usize], cache: &mut HashMap<(String, String), usize>) -> usize {
        let key: (String, String) = (data.to_owned(), runs.iter().join(","));
        if cache.contains_key(&key) {
            return cache[&key];
        }

        let stripped = data.trim_start_matches(".");

        let answer: usize = if stripped.is_empty() {
            let a = if runs.len() == 0 { 1 } else { 0 };
            a
        } else if runs.is_empty() {
            let a = if stripped.contains('#') { 0 } else { 1 };
            a
        } else if stripped.chars().nth(0).unwrap() == '#' {
            let a = if stripped.len() < runs[0] || stripped[0..runs[0]].contains('.') {
                0 // not enough space
            } else if stripped.len() == runs[0] {
                let a = if runs.len() == 1 { 1usize } else { 0usize }; //perfect match
                a
            } else if stripped.chars().nth(runs[0]).unwrap_or('!') == '#' {
                0  // too many springs
            } else {
                count_solutions_better(&stripped[runs[0] + 1..], &runs[1..], cache)
            };
            a
        } else {
            let aa = count_solutions_better(&("#".to_owned() + &stripped[1..]), runs, cache);
            let bb = count_solutions_better(&stripped[1..], runs, cache);
            aa + bb
        };

        cache.insert(key, answer);
        return answer;
    }

    fn parse_line(input: &str) -> (&str, Vec<usize>) {
        let split = input.split(" ").collect_vec();

        let data = split[0];
        let parity_str = split[1];

        let parity = parity_str.split(",").map(|n| n.parse::<usize>().unwrap()).collect_vec();

        return (data, parity);
    }

    fn parse_lines(input: &str) -> Vec<(&str, Vec<usize>)> {
        return input
            .lines()
            .map(|l| parse_line(l))
            .collect_vec();
    }

    fn apply_p1(input: &str) -> usize {
        return parse_lines(input)
            .iter().map(|(data, parity)| count_solutions(data, parity))
            .sum();
    }

    fn apply_p2(input: &str) -> usize {
        return parse_lines(input)
            .iter()
            .map(|(data, parity)| expand(data, parity))
            .map(|(data, parity)| count_solutions_better(&data, &parity, &mut HashMap::new()))
            .sum();
    }

    fn expand<'a>(data: &'a str, parity: &'a Vec<usize>) -> (String, Vec<usize>) {
        let new_data = vec![data].repeat(5).join("?");

        let new_parity = vec![parity].repeat(5).into_iter().flat_map(|v| v.iter().cloned()).collect();

        return (new_data, new_parity);
    }

    #[test]
    fn test_count_solutions() {
        assert_eq!(10, count_solutions("?###????????", &vec![3, 2, 1]));
        assert_eq!(1, count_solutions("???.###", &vec![1, 1, 3]));
        assert_eq!(4, count_solutions(".??..??...?##.", &vec![1, 1, 3]));
        assert_eq!(1, count_solutions("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6]));
        assert_eq!(1, count_solutions("????.#...#...", &vec![4, 1, 1]));
        assert_eq!(4, count_solutions("????.######..#####.", &vec![1, 6, 5]));
        assert_eq!(10, count_solutions("?###????????", &vec![3, 2, 1]));
    }

    fn run(d: &str, v: &Vec<usize>) -> usize {
        let (dd, vv) = expand(d, v);
        return count_solutions_better(&dd, &vv, &mut HashMap::new());
    }

    #[test]
    fn test_count_solutions_better() {
        assert_eq!(1, run("???.###", &vec![1, 1, 3]));
        assert_eq!(16384, run(".??..??...?##.", &vec![1, 1, 3]));
        assert_eq!(1, run("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6]));
        assert_eq!(16, run("????.#...#...", &vec![4, 1, 1]));
        assert_eq!(2500, run("????.######..#####.", &vec![1, 6, 5]));
        assert_eq!(506250, run("?###????????", &vec![3, 2, 1]));
    }

    #[test]
    fn test_is_correct() {
        assert_eq!(true, is_correct(".###.##.#...", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###.##..#..", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###.##...#.", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###.##....#", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###..##.#..", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###..##..#.", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###..##...#", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###...##.#.", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###...##..#", &vec![3, 2, 1]));
        assert_eq!(true, is_correct(".###....##.#", &vec![3, 2, 1]));


        assert_eq!(false, is_correct(".###.##.#.##", &vec![3, 2, 1]));


        assert_eq!(false, is_correct(".####...##.#", &vec![3, 2, 1]));
        assert_eq!(false, is_correct(".####.#.##.#", &vec![3, 2, 1]));
    }

    #[test]
    fn test_expand() {
        assert_eq!((".#?.#?.#?.#?.#".to_owned(), vec![1usize, 1usize, 1usize, 1usize, 1usize]), expand(".#", &vec![1]))
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
        let data = read_data_file(12, "input.txt");

        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(12, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}