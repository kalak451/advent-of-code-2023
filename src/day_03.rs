#[cfg(test)]
mod day01 {
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::grid::Grid;
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

    fn subgrid_contains_symbol(grid: &Grid, y:usize, start: usize, end: usize) -> Vec<(usize, usize, &char)> {
        let sym_start_x = if start==0 {start} else {start-1};
        let sym_end_x = end + 1;

        let sym_start_y = if y == 0 {y} else {y-1};
        let sym_end_y = y+1;

        let mut res = Vec::new();

        for sy in sym_start_y..=sym_end_y {
            for sx in sym_start_x..=sym_end_x {
                let c = grid.get(sx, sy).unwrap_or(&'.');
                if !c.is_ascii_digit() && c != &'.' {
                    res.push((sy, sx, c));
                }
            }
        }

        return res;
    }

    fn process_part_1(grid: Grid) -> (i64, i64) {
        let mut p1_sum: i64 = 0;
        let mut found = false;
        let mut start = 0;
        let mut end = 0;

        let mut p2_index: HashMap<(usize, usize), Vec<i64>> = HashMap::new();

        let (x_bounds, y_bounds) = grid.bounds();

        for y in y_bounds.start..y_bounds.end {
            for x in x_bounds.start..x_bounds.end {
                if !found {
                    if grid.get(x,y).unwrap().is_ascii_digit() {
                        found = true;
                        start = x;
                        end = x;
                    }
                } else {
                    if !grid.pos_is_ascii_digit(x,y) || grid.is_x_bound(x) {
                        found = false;
                        if grid.is_x_bound(x) && grid.pos_is_ascii_digit(x,y) {
                            end = x;
                        }

                        let found_symbols = subgrid_contains_symbol(&grid, y, start, end);
                        if !found_symbols.is_empty() {
                            let value = grid.get_x_slice(start..=end, y)
                                .unwrap()
                                .iter()
                                .join("")
                                .parse::<i64>()
                                .unwrap();

                            p1_sum += value;

                            found_symbols.iter()
                                .filter(|s| s.2 == &'*')
                                .for_each(|s| {
                                    let parts = p2_index.entry((s.0, s.1)).or_insert(Vec::new());
                                    parts.push(value);
                                })
                        }

                    } else {
                        end = x;
                    }
                }
            }
        }

        let p2_sum = p2_index.values()
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .sum();

        return (p1_sum, p2_sum);
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

    fn to_grid(data: &str) -> Grid {
        return Grid::from_lines(data);
    }

    #[test]
    fn part_2() {
        let data = read_data_file(3, "input.txt");
        let grid = to_grid(&data);

        let aaa = process_part_1(grid).1;
        println!("Answer: {aaa:?}");
    }
}