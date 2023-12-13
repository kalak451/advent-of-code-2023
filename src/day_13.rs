#[cfg(test)]
mod day13 {
    use std::ops::Range;
    use itertools::Itertools;
    use crate::grid::Grid;

    use crate::read_data_file;

    static SAMPLE_1: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    fn is_max_pair(p: &(usize, usize), bound: &Range<usize>) -> bool {
        let v = [p.0, p.1];
        if v.iter().any(|t| *t == 0) {
            return true;
        }

        if v.iter().any(|t| *t == bound.end - 1) {
            return true;
        }

        return false;
    }

    fn increase_pair(p: (usize, usize)) -> (usize, usize) {
        return (p.0 - 1, p.1 + 1);
    }

    fn build_initial_pairs(bound: &Range<usize>) -> Vec<(usize, usize)> {
        return bound.to_owned().tuple_windows::<(_, _)>().collect_vec();
    }

    fn find_vertical_mirror_line(grid: &Grid, ignore: Option<usize>) -> Option<usize> {
        let (x_bounds, y_bounds) = grid.bounds();

        let mut pairs = build_initial_pairs(&x_bounds);

        if ignore.is_some() {
            pairs = pairs
                .into_iter()
                .filter(|p| p.0 != ignore.unwrap())
                .collect_vec();
        }

        while !pairs.is_empty() {
            for y in y_bounds.to_owned() {
                pairs = pairs
                    .into_iter()
                    .filter(|(a, b)| {
                        let aa = *grid.get(*a, y).unwrap();
                        let bb = *grid.get(*b, y).unwrap();

                        return aa == bb;
                    })
                    .collect_vec();
            }

            if pairs.iter().any(|p| is_max_pair(p, &x_bounds)) {
                break;
            }

            pairs = pairs.into_iter().map(|p| increase_pair(p)).collect_vec();
        }

        pairs = pairs.into_iter().filter(|p| is_max_pair(&p, &x_bounds)).collect_vec();

        if pairs.is_empty() {
            return None;
        }

        if pairs.len() == 1 {
            let p = pairs[0];
            let sym_start_idx = (p.1 - p.0) / 2 + p.0;
            return Some(sym_start_idx + 1);
        } else {
            panic!("Should never be here!");
        }
    }

    fn find_horizontal_mirror_line(grid: &Grid, ignore: Option<usize>) -> Option<usize> {
        let (x_bounds, y_bounds) = grid.bounds();

        let mut pairs = build_initial_pairs(&y_bounds);

        if ignore.is_some() {
            pairs = pairs
                .into_iter()
                .filter(|p| p.0 != ignore.unwrap())
                .collect_vec();
        }

        while !pairs.is_empty() {
            for x in x_bounds.to_owned() {
                pairs = pairs
                    .into_iter()
                    .filter(|(a, b)| {
                        let aa = *grid.get(x, *a).unwrap();
                        let bb = *grid.get(x, *b).unwrap();

                        return aa == bb;
                    })
                    .collect_vec();
            }

            if pairs.iter().any(|p| is_max_pair(p, &y_bounds)) {
                break;
            }

            pairs = pairs.into_iter().map(|p| increase_pair(p)).collect_vec();
        }

        pairs = pairs.into_iter().filter(|p| is_max_pair(&p, &y_bounds)).collect_vec();

        if pairs.is_empty() {
            return None;
        }

        if pairs.len() == 1 {
            let p = pairs[0];
            let sym_start_idx = (p.1 - p.0) / 2 + p.0;
            return Some(sym_start_idx + 1);
        } else {
            panic!("Should never be here!");
        }
    }

    fn get_sym_value(grid: &Grid) -> usize {
        let vr = find_vertical_mirror_line(grid, None);
        if vr.is_some() {
            return vr.unwrap();
        }

        let hr = find_horizontal_mirror_line(grid, None);
        if hr.is_some() {
            return hr.unwrap() * 100;
        }

        return 0;
    }

    fn get_sym_value_p2(grid: &mut Grid) -> usize {
        let vr = find_vertical_mirror_line(grid, None).map(|x| x - 1);
        let hr = find_horizontal_mirror_line(grid, None).map(|x| x - 1);

        let (x_bounds, y_bounds) = grid.bounds();

        let mut prev_smudge: Option<(usize, usize)> = None;

        for y in y_bounds.to_owned() {
            for x in x_bounds.to_owned() {
                if prev_smudge.is_some() {
                    let (xx,yy) = prev_smudge.unwrap();
                    flip(grid, xx, yy);
                }

                prev_smudge = Some((x,y));
                flip(grid, x, y);

                let vrr = find_vertical_mirror_line(grid, vr);
                if vrr.is_some() {
                    return vrr.unwrap();
                }

                let hrr = find_horizontal_mirror_line(grid, hr);
                if hrr.is_some() {
                    return hrr.unwrap() * 100;
                }
            }
        }

        return 0;
    }

    fn flip(grid: &mut Grid, x: usize, y: usize) {
        let c = *grid.get(x,y).unwrap();

        if c == '.' {
            grid.set(x,y, '#');
        }

        if c == '#' {
            grid.set(x,y, '.');
        }

    }

    fn apply_p1(data: &str) -> usize {
        data
            .split("\n\n")
            .map(|pan| Grid::from_lines(pan))
            .enumerate()
            .map(|(idx, g)| {
                println!("Grid: {idx}");
                get_sym_value(&g)
            })
            .sum()
    }

    fn apply_p2(data: &str) -> usize {
        data
            .split("\n\n")
            .map(|pan| Grid::from_lines(pan))
            .enumerate()
            .map(|(idx, mut g)| {
                println!("Grid: {idx}");
                get_sym_value_p2(&mut g)
            })
            .sum()
    }

    #[test]
    fn test_find_vertical_mirror_line() {
        let data = SAMPLE_1;
        let panels = data.split("\n\n").collect_vec();

        let grid = Grid::from_lines(panels[0]);
        let answer = find_vertical_mirror_line(&grid, None);
        assert_eq!(Some(5), answer);

        let grid2 = Grid::from_lines(panels[1]);
        let answer2 = find_vertical_mirror_line(&grid2, None);
        assert_eq!(None, answer2);
    }

    #[test]
    fn test_find_horizontal_mirror_line() {
        let data = SAMPLE_1;
        let panels = data.split("\n\n").collect_vec();

        let grid = Grid::from_lines(panels[0]);
        let answer = find_horizontal_mirror_line(&grid, None);
        assert_eq!(None, answer);

        let grid2 = Grid::from_lines(panels[1]);
        let answer2 = find_horizontal_mirror_line(&grid2, None);
        assert_eq!(Some(4), answer2);
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
        let data = read_data_file(13, "input.txt");

        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(13, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}