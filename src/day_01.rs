#[cfg(test)]
mod day01 {
    use regex::Regex;

    use crate::read_data_file;

    #[test]
    fn part_1() {
        let data = read_data_file(1, "input.txt");

        let rgx = Regex::new(r"[^0-9]").unwrap();

        let digits_only: i32 = data.lines()
            .map(|l| rgx.replace_all(l, "") )
            .map(|s| {
                let a = String::from(s.chars().next().unwrap());
                let b = String::from(s.chars().last().unwrap());
                return a + &b;
            })
            .map(|s| s.parse::<i32>().unwrap())
            .sum();


        println!("d = {digits_only:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(1, "input.txt");

        let search_words = vec!(
            ("one", 1),
            ("1", 1),
            ("two", 2),
            ("2", 2),
            ("three", 3),
            ("3", 3),
            ("four", 4),
            ("4", 4),
            ("five", 5),
            ("5", 5),
            ("six", 6),
            ("6", 6),
            ("seven", 7),
            ("7", 7),
            ("eight", 8),
            ("8", 8),
            ("nine", 9),
            ("9", 9),
        );

        // println!("data = {data}");

        let answer: i32 = data.lines()
            .map(|l| {
                let first = search_words.iter()
                    .map(|t| l.find(t.0).map(|p| (p, t.1)))
                    .flatten()
                    .min_by_key(|t| t.0)
                    .map(|t| t.1)
                    .map(|d| d.to_string())
                    .unwrap();

                let second = search_words.iter()
                    .map(|t| l.rfind(t.0).map(|p| (p, t.1)))
                    .flatten()
                    .max_by_key(|t| t.0)
                    .map(|t| t.1)
                    .map(|d| d.to_string())
                    .unwrap();

                let res = first + &second;

                return res;
            })
            .map(|s| s.parse::<i32>().unwrap())
            .sum();


        println!("answer = {answer:?}");
    }
}