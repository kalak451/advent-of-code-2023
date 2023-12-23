#[cfg(test)]
mod day21 {
    use std::collections::{HashSet, VecDeque};


    use crate::grid::{DIR, Grid};
    use crate::grid::DIR::{EAST, NORTH, SOUTH, WEST};
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    fn apply_p1(input: &str, max_steps: usize) -> usize {
        let grid = Grid::from_lines(input);
        let start = grid.find_first('S').unwrap();

        return fill(&grid, start.x as i64, start.y as i64, max_steps);
    }

    fn fill (grid: &Grid, start_x: i64, start_y: i64, start_steps: usize) -> usize {
        let mut ans = HashSet::new();
        let mut seen: HashSet<(i64,i64)> = HashSet::from_iter([(start_x,start_y)]);
        let mut q = VecDeque::from_iter([(start_x, start_y, start_steps)]);

        while let Some((x,y,steps)) = q.pop_front() {
            if steps % 2 == 0 {
                ans.insert((x,y));
            }

            if steps == 0 {
                continue;
            } else {
                for (nx, ny) in [(x+1, y), (x-1,y), (x, y+1), (x, y-1)] {
                    if !seen.contains(&(nx,ny)) {
                        if let Some(&c) = grid.get_i(nx as i32, ny as i32) {
                            if c != '#' {
                                seen.insert((nx, ny));
                                q.push_back((nx, ny, steps - 1));
                            }
                        }
                    }
                }
            }

        }

        return ans.len();
    }

    fn apply_p2(input: &str, max_steps: i64) -> usize {
        let grid = Grid::from_lines(input);
        let start = grid.find_first('S').unwrap();

        let goal = 26501365i64;

        let mut p_len = 0usize;

        let mut stage_results = HashSet::from_iter([(start.x as i64, start.y as i64)]);

        for i in 1..=max_steps {
            let mut res = HashSet::new();
            let starts = &stage_results;

            for p in starts {
                for d in [NORTH, SOUTH, EAST, WEST] {
                    let np = mv(p, d);

                    let (x, y) = scale_to_grid(&grid, &np);

                    if *grid.get(x, y).unwrap() != '#' {
                        res.insert(np);
                    }
                }
            }

            stage_results = res;

            if i % (grid.x_size as i64) == goal % (grid.x_size as i64) {
                let l = stage_results.len();
                let diff = l - p_len;
                let i_n = i / grid.x_size as i64;
                let s = format!("i = {i}, len={l}, diff={diff}, i_n={i_n}");
                println!("{s}");
                p_len = l;
            }
        }

        return stage_results.len();
    }

    fn scale_to_grid(grid: &Grid, p: &(i64, i64)) -> (usize, usize) {
        let x = p.0 % grid.x_size as i64;
        let y = p.1 % grid.y_size as i64;

        let xx = if x < 0 {
            (grid.x_size as i64 + x) as usize
        } else {
            x as usize
        };

        let yy = if y < 0 {
            (grid.y_size as i64 + y) as usize
        } else {
            y as usize
        };

        return (xx, yy);
    }

    fn mv(p: &(i64, i64), dir: DIR) -> (i64, i64) {
        match dir {
            NORTH => (p.0 - 1, p.1),
            SOUTH => (p.0 + 1, p.1),
            EAST => (p.0, p.1 + 1),
            WEST => (p.0, p.1 - 1),
        }
    }

    #[test]
    fn sample_1_p1() {
        let data = SAMPLE_1;

        let results = apply_p1(data, 6);
        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_1_p2() {
        let data = SAMPLE_1;

        // assert_eq!(16, apply_p2(data, 6));
        // assert_eq!(50, apply_p2(data, 10));
        assert_eq!(1594, apply_p2(data, 50));
        // assert_eq!(6536, apply_p2(data, 100));
        // assert_eq!(167004, apply_p2(data, 500));
        // assert_eq!(668697, apply_p2(data, 1000));
        // assert_eq!(16733044, apply_p2(data, 5000));

        let results = apply_p2(data, 6);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(21, "input.txt");


        let results = apply_p1(&data, 64);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        // let data = read_data_file(21, "input.txt");

        let a0 = 3699f64;
        let a1 = 33137f64;
        let a2 = 91951f64;

        let n = (26501365f64 / 131f64).floor();

        let b0 = a0;
        let b1 = a1 - a0;
        let b2 = a2 - a1;


        let results = b0 + b1*n + (n*(n-1f64)/2f64).floor()*(b2-b1);

        println!("Answer: {results:?}");
        // apply_p2(&data, 1000);
    }
}