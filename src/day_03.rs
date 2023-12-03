#[cfg(test)]
mod day01 {
    use std::cmp;
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::read_data_file;

    static SAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    fn subgrid_contains_symbol(grid: &Vec<Vec<char>>, y:i32, start: i32, end: i32) -> Vec<(usize, usize, char)> {
        let sym_start_x = cmp::max(start-1, 0) as usize;
        let sym_end_x = cmp::min(end+1, grid[y as usize].len() as i32 -1) as usize;

        let sym_start_y = cmp::max(y-1, 0) as usize;
        let sym_end_y = cmp::min(y+1, (grid.len()-1) as i32) as usize;

        let mut res = Vec::new();

        for sy in sym_start_y..=sym_end_y {
            for sx in sym_start_x..=sym_end_x {
                let c = grid[sy][sx];
                if !c.is_ascii_digit() && c != '.' {
                    res.push((sy,sx,c));
                }
            }
        }

        return res;
    }

    fn process_part_1(grid: Vec<Vec<char>>) -> (i64, i64) {
        let mut sum: i64 = 0;
        let mut found = false;
        let mut start = 0;
        let mut end = 0;

        let mut sym: HashMap<(usize, usize), Vec<i64>> = HashMap::new();

        for y in 0..grid.len() {
            for x in 0.. grid[y].len() {
                if !found {
                    if grid[y][x].is_ascii_digit() {
                        found = true;
                        start = x;
                        end = x;
                    }
                } else {
                    if !grid[y][x].is_ascii_digit() || x == grid[y].len() - 1 {
                        found = false;
                        if x == grid[y].len() - 1 && grid[y][x].is_ascii_digit() {
                            end = x;
                        }

                        let symbols = subgrid_contains_symbol(&grid, y as i32, start as i32, end as i32);
                        if !symbols.is_empty() {
                            let v = grid[y].get(start..=end).unwrap().iter().join("").parse::<i64>().unwrap();
                            sum += v;

                            symbols.iter()
                                .filter(|s| s.2 == '*')
                                .for_each(|s| {
                                    let e = sym.entry((s.0, s.1)).or_insert(Vec::new());
                                    e.push(v);
                                })
                        }

                    } else {
                        end = x;
                    }
                }
            }
        }

        let ratio_sum = sym.values()
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .sum();

        return (sum, ratio_sum);
    }

    #[test]
    fn sample() {
        let data = SAMPLE;
        let grid = to_grid(data);

        let aaa = process_part_1(grid).0;
        println!("Answer: {aaa:?}");
    }

    #[test]
    fn sample_p2() {
        let data = SAMPLE;
        let grid = to_grid(data);

        let aaa = process_part_1(grid).1;
        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(3, "input.txt");
        let grid = to_grid(&data);

        let aaa = process_part_1(grid).0;
        println!("Answer: {aaa:?}");
    }

    fn to_grid(data: &str) -> Vec<Vec<char>> {
        let grid: Vec<Vec<char>> = data
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        grid
    }

    #[test]
    fn part_2() {
        let data = read_data_file(3, "input.txt");
        let grid = to_grid(&data);

        let aaa = process_part_1(grid).1;
        println!("Answer: {aaa:?}");
    }
}