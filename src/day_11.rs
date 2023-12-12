#[cfg(test)]
mod day11 {
    use std::cmp::{max, min};
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::grid::{Grid, Point};
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    fn calculate_open_columns(grid: &Grid) -> HashSet<usize> {
        let (x_bounds, y_bounds) = grid.bounds();

        let mut results = HashSet::new();

        for x in x_bounds.to_owned() {
            let mut empty = true;
            for y in y_bounds.to_owned() {
                if *grid.get(x, y).unwrap() != '.' {
                    empty = false;
                    break;
                }
            }
            if empty {
                results.insert(x);
            }
        }

        return results;
    }

    fn calculate_open_rows(grid: &Grid) -> HashSet<usize> {
        let (x_bounds, y_bounds) = grid.bounds();

        let mut results = HashSet::new();

        for y in y_bounds.to_owned() {
            let mut empty = true;
            for x in x_bounds.to_owned() {
                if *grid.get(x, y).unwrap() != '.' {
                    empty = false;
                    break;
                }
            }
            if empty {
                results.insert(y);
            }
        }

        return results;
    }

    fn calculate_empties(grid: &Grid) -> (HashSet<usize>, HashSet<usize>) {
        return (
            calculate_open_columns(grid),
            calculate_open_rows(grid)
        );
    }

    fn find_galaxies(grid: &Grid) -> Vec<Point> {
        let mut results = vec![];

        let (x_bounds, y_bounds) = grid.bounds();
        for x in x_bounds.to_owned() {
            for y in y_bounds.to_owned() {
                let p = Point::new(x as i32, y as i32);
                let c = grid.get_point(&p).unwrap();

                if *c == '#' {
                    results.push(p);
                }
            }
        }

        return results;
    }

    fn map_distance(start: usize, end: usize, doubles: &HashSet<usize>, empty_size: usize) -> usize {
        let mut dist = 0usize;

        let s = min(start, end);
        let e = max(start, end);

        for i in (s + 1)..=e {
            if doubles.contains(&i) {
                dist = dist + empty_size;
            } else {
                dist = dist + 1;
            }
        }

        return dist;
    }

    fn calc_distance(p1: &Point, p2: &Point, cols: &HashSet<usize>, rows: &HashSet<usize>, empty_size: usize) -> usize {
        let x_dist = map_distance(p1.x, p2.x, &cols, empty_size);
        let y_dist = map_distance(p1.y, p2.y, &rows, empty_size);
        return x_dist + y_dist;
    }

    fn apply_p1(grid: &Grid, empty_size: usize) -> usize {
        let (cols, rows) = calculate_empties(grid);
        let galaxies = find_galaxies(grid);
        let galaxy_combos = galaxies.iter().combinations(2).collect_vec();

        let total_distance = galaxy_combos
            .iter()
            .map(|pts| calc_distance(pts[0], pts[1], &cols, &rows, empty_size))
            .sum();

        return total_distance;
    }

    #[test]
    fn test_distance() {
        let data = SAMPLE_1;
        let grid = Grid::from_lines(data);
        let (cols, rows) = calculate_empties(&grid);

        let dist_5_9 = calc_distance(&Point::new(1,5), &Point::new(4,9), &cols, &rows, 2usize);
        assert_eq!(9, dist_5_9);

        let dist_1_7 = calc_distance(&Point::new(3,0), &Point::new(7,8), &cols, &rows, 2usize);
        assert_eq!(15, dist_1_7);

        let dist_3_6 = calc_distance(&Point::new(0, 2), &Point::new(9, 6), &cols, &rows, 2usize);
        assert_eq!(17, dist_3_6);

        let dist_8_9 = calc_distance(&Point::new(0, 9), &Point::new(4, 9), &cols, &rows, 2usize);
        assert_eq!(5, dist_8_9);
    }

    #[test]
    fn sample_1_p1() {
        let data = SAMPLE_1;
        let grid = Grid::from_lines(data);

        let results = apply_p1(&grid, 2usize);
        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_1_p2() {
        let data = SAMPLE_1;
        let grid = Grid::from_lines(data);

        let results = apply_p1(&grid, 10usize);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(11, "input.txt");
        let grid = Grid::from_lines(&data);

        let results = apply_p1(&grid, 2usize);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(11, "input.txt");
        let grid = Grid::from_lines(&data);

        let results = apply_p1(&grid, 1000000usize);
        println!("Answer: {results:?}");
    }
}