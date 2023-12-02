#[cfg(test)]
mod day01 {
    use std::cmp;
    use regex::Regex;

    use crate::read_data_file;

    fn extract_value(s: &str, r: &Regex) -> Option<String> {
        return r.captures(s).map(|c| (&c[1]).to_owned());
    }

    fn extract_data(data: String) -> Vec<Vec<(Option<i32>, Option<i32>, Option<i32>)>> {
        let game_regex = Regex::new(r"^Game \d+: (.*)$").unwrap();
        let red_regex = Regex::new(r"(\d+) red").unwrap();
        let blue_regex = Regex::new(r"(\d+) blue").unwrap();
        let green_regex = Regex::new(r"(\d+) green").unwrap();

        return data
            .lines()
            .map(|l| extract_value(l, &game_regex).unwrap())
            .map(|s| s
                .split(";")
                .map(|pg| {
                    let red = extract_value(pg, &red_regex).map(|x| x.parse::<i32>().unwrap());
                    let green = extract_value(pg, &green_regex).map(|x| x.parse::<i32>().unwrap());
                    let blue = extract_value(pg, &blue_regex).map(|x| x.parse::<i32>().unwrap());
                    return (red, green, blue);
                })
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();
    }

    #[test]
    fn part_1() {
        let data = read_data_file(2, "input.txt");
//         let data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// "#;
        let game_data = extract_data(data.to_owned());

        let aaa: usize = game_data
            .iter()
            .map(|game| game
                .iter()
                .map(|t| (t.0.unwrap_or(0), t.1.unwrap_or(0), t.2.unwrap_or(0)))
                .fold((0, 0, 0), |acc, t| (cmp::max(acc.0,  t.0), cmp::max(acc.1, t.1), cmp::max(acc.2,t.2)))
            )
            .enumerate()
            .filter(|tt| {
                let t = tt.1;
                if t.0 <= 12 && t.1 <= 13 && t.2 <= 14 {
                    return true;
                }

                return false;
            })
            .map(|tt| tt.0 + 1)
            // .collect::<Vec<_>>();
            .sum();

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(2, "input.txt");
//         let data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// "#;
        let game_data = extract_data(data.to_owned());

        let aaa: i32 = game_data
            .iter()
            .map(|game| game
                .iter()
                .map(|t| (t.0.unwrap_or(0), t.1.unwrap_or(0), t.2.unwrap_or(0)))
                .fold((0, 0, 0), |acc, t| (cmp::max(acc.0,  t.0), cmp::max(acc.1, t.1), cmp::max(acc.2,t.2)))
            )
            .map(|t| t.0 * t.1 * t.2 )
            .sum();

        println!("Answer: {aaa:?}");
    }
}