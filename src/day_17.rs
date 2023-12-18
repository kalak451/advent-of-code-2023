#[cfg(test)]
mod day17 {
    use std::cmp;

    use DIR::SOUTH;

    use crate::{read_data_file, shortest_path};
    use crate::grid::{DIR, Grid, Point};
    use crate::grid::DIR::{EAST, NORTH, WEST};

    static SAMPLE_1: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    fn dir_choices(current_dir: Option<DIR>) -> Vec<DIR> {
        match current_dir {
            None => vec![NORTH, SOUTH, EAST, WEST],
            Some(NORTH) => vec![EAST, WEST],
            Some(SOUTH) => vec![EAST, WEST],
            Some(EAST) => vec![NORTH, SOUTH],
            Some(WEST) => vec![NORTH, SOUTH]
        }
    }

    fn point_choices(grid: &Grid, current_point: Point, last_dir: Option<DIR>, min_step: i32, max_step: i32) -> Vec<((Point, Option<DIR>), i64)> {
        let mut res = vec![];
        for d in dir_choices(last_dir) {
            let mut np = Some(current_point);
            let mut cost = 0;
            for dist in 1..=max_step {
                np = grid.try_move(&np.unwrap(), &d);
                if np.is_some() {
                    cost = cost + grid.get_point(&np.unwrap()).unwrap().to_digit(10).unwrap() as i64;
                    if dist >= min_step {
                        res.push(((np.unwrap(), Some(d)), cost));
                    }
                } else {
                    break;
                }
            }
        }

        return res;
    }

    fn run_grid(input: &str, min_step: i32, max_step: i32) -> i64 {
        let grid = Grid::from_lines(input);

        let costs = shortest_path(
            (Point::new(0,0), None),
            | (p, d) | point_choices(&grid, p, d, min_step, max_step)
        );

        let max_p = Point::new(grid.x_size as i32 - 1, grid.y_size as i32 - 1);
        return cmp::min(
            *costs.get(&(max_p, Some(SOUTH))).unwrap_or(&i64::MAX),
            *costs.get(&(max_p, Some(EAST))).unwrap_or(&i64::MAX),
        );
    }

    fn apply_p1(input: &str) -> i64 {
        return run_grid(input, 1, 3);
    }

    fn apply_p2(input: &str) -> i64 {
        return run_grid(input, 4, 10);
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
        let data = read_data_file(17, "input.txt");


        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(17, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}