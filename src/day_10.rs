#[cfg(test)]
mod day10 {
    use std::collections::{HashMap, HashSet};
    use std::collections::hash_map::RandomState;

    use itertools::Itertools;

    use crate::grid::{Grid, Point};
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"-L|F7
7F-7|
L|7||
-L-J|
L|-JF"#;
    static SAMPLE_2: &str = r#"..F7.
.FJ|.
FJ.L7
|F--J
LJ..."#;

    static SAMPLE_3: &str = r#"...........
.F-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
    static SAMPLE_4: &str = r#"..........
.F------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."#;
    static SAMPLE_5: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJF7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
    static SAMPLE_6: &str = r#"FF7F7F7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

    fn build_move_index() -> HashMap<char, Vec<Vec<i32>>> {
        let north = vec![0, -1];
        let east = vec![1, 0];
        let south = vec![0, 1];
        let west = vec![-1, 0];

        return HashMap::from([
            ('|', vec![north.to_owned(), south.to_owned()]),
            ('-', vec![east.to_owned(), west.to_owned()]),
            ('L', vec![north.to_owned(), east.to_owned()]),
            ('J', vec![north.to_owned(), west.to_owned()]),
            ('7', vec![south.to_owned(), west.to_owned()]),
            ('F', vec![south.to_owned(), east.to_owned()]),
            ('.', vec![]),
        ]);
    }

    fn follow_path(grid: &Grid, start: &Point) -> Vec<Point> {
        let move_index = build_move_index();
        let starting_tile = grid.get_point(start).unwrap();
        let starting_moves = move_index.get(starting_tile).unwrap();
        let starting_move = start.apply_dir_vector(&starting_moves[0]).unwrap();

        let mut path: Vec<Point> = vec![start.to_owned(), starting_move.to_owned()];

        while path.first().unwrap() != path.last().unwrap() {
            let current = path.get(path.len() - 1).unwrap();
            let prev = path.get(path.len() - 2).unwrap();
            let current_tile = grid.get_point(current).unwrap();
            let moves = move_index.get(current_tile).unwrap();
            let mv = moves.iter()
                .map(|m| current.apply_dir_vector(m).unwrap())
                .filter(|new| *new != *prev)
                .exactly_one()
                .unwrap();

            path.push(mv);
        }

        return path;
    }

    fn count_inside(grid: &Grid, path: &Vec<Point>) -> usize {
        let path_points: HashSet<Point, RandomState> = HashSet::from_iter(path.iter().cloned());

        let mut inside_points: Vec<Point> = vec![];
        let mut inside_count = 0usize;


        let (x_range, y_range) = grid.bounds();

        for y in y_range.to_owned() {
            for x in x_range.to_owned() {
                let p = Point::new(x as i32, y as i32);
                if !path_points.contains(&p) {
                    let mut count = 0;

                    let mut xx = p.x;
                    let mut yy = p.y;

                    while xx < x_range.end && yy < y_range.end {
                        let c = grid.get(xx, yy).unwrap();
                        let pp = Point::new(xx as i32,yy as i32);

                        if path_points.contains(&pp) && *c != 'L' && *c != '7' {
                            count = count + 1;
                        }

                        xx = xx + 1;
                        yy = yy + 1;
                    }


                    if count % 2 == 1 {
                        inside_count = inside_count + 1;
                        inside_points.push(p);
                    }
                }
            }
        }

        return inside_count;
    }


    #[test]
    fn sample_1_p1() {
        let data = SAMPLE_1;
        let start = Point::new(1, 1);
        let grid = Grid::from_lines(data);

        let path = follow_path(&grid, &start);

        let results: usize = path.len() / 2;

        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_2_p1() {
        let data = SAMPLE_2;
        let start = Point::new(0, 2);
        let grid = Grid::from_lines(data);

        let path = follow_path(&grid, &start);

        let results: usize = path.len() / 2;

        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_3_p2() {
        let data = SAMPLE_3;
        let start = Point::new(1, 1);
        let grid = Grid::from_lines(data);

        let path = follow_path(&grid, &start);

        let results = count_inside(&grid, &path);

        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_4_p2() {
        let data = SAMPLE_4;
        let start = Point::new(1, 1);
        let grid = Grid::from_lines(data);

        let path = follow_path(&grid, &start);

        let results = count_inside(&grid, &path);

        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_5_p2() {
        let data = SAMPLE_5;
        let start = Point::new(12, 4);
        let grid = Grid::from_lines(data);

        let path = follow_path(&grid, &start);

        let results = count_inside(&grid, &path);

        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_6_p2() {
        let data = SAMPLE_6;
        let start = Point::new(4, 0);
        let grid = Grid::from_lines(data);

        let path = follow_path(&grid, &start);

        let results = count_inside(&grid, &path);

        println!("Answer: {results:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(10, "input-fixed.txt");
        let start = Point::new(34, 114);
        let grid = Grid::from_lines(&data);

        let path = follow_path(&grid, &start);

        let results: usize = path.len() / 2;

        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(10, "input-fixed.txt");
        let start = Point::new(34, 114);
        let grid = Grid::from_lines(&data);

        let path = follow_path(&grid, &start);

        let results = count_inside(&grid, &path);

        println!("Answer: {results:?}");
    }
}