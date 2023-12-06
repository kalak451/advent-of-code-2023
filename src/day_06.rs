#[cfg(test)]
mod day06 {
    use itertools::Itertools;

    #[derive(Debug)]
    struct RaceRecord {
        time: i64,
        distance: i64
    }

    fn get_race_history() -> Vec<RaceRecord> {
        return vec![
            RaceRecord {time: 44, distance: 208},
            RaceRecord {time: 80, distance: 1581},
            RaceRecord {time: 65, distance: 1050},
            RaceRecord {time: 72, distance: 1102},
        ];
    }

    fn get_sample() -> Vec<RaceRecord> {
        return vec![
            RaceRecord {time: 7, distance: 9},
            RaceRecord {time: 15, distance: 40},
            RaceRecord {time: 30, distance: 200},
        ];
    }



    fn calc_win_count(max_time: i64, max_distance: i64) -> i64 {
        let mut min_loss_time = max_time;

        for t in 0..=max_time {
            let run_time = max_time - t;
            let distance = run_time * t;
            if distance <= max_distance {
                min_loss_time = t;
            } else {
                break
            }
        }

        let mut max_loss_time = 0;
        for t in (0..=max_time).rev() {
            let run_time = max_time - t;
            let distance = run_time * t;
            if distance <= max_distance {
                max_loss_time = t;
            } else {
                break
            }
        }

        let result = (max_loss_time - 1) - (min_loss_time + 1) + 1;
        println!("Result: {result}, Max_Loss_time: {max_loss_time}, min_loss_time {min_loss_time}");
        return result;
    }

    fn generate_part_1(input: &Vec<RaceRecord>) -> i64 {
        return input
        .iter()
            .map(|record| {
                return calc_win_count(record.time, record.distance);
            })
            .product1()
            .unwrap();
    }

    #[test]
    fn sample_p1() {
        let aaa: i64 = generate_part_1(&get_sample());

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn sample_p2() {
        let aaa: i64 = generate_part_1(&vec![RaceRecord{time: 71530, distance: 940200}]);

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_1() {
        let aaa: i64 = generate_part_1(&get_race_history());

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_2() {
        let aaa: i64 = generate_part_1(&vec![RaceRecord{time: 44806572, distance: 208158110501102}]);

        println!("Answer: {aaa:?}");
    }
}