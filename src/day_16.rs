#[cfg(test)]
mod day16 {
    use std::collections::{HashSet, VecDeque};

    use DIR::{EAST, NORTH, SOUTH};

    use crate::grid::{DIR, Grid, Point};
    use crate::grid::DIR::WEST;
    use crate::read_data_file;

    static SAMPLE_1: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    fn interact(c: char, dir: &DIR) -> Vec<DIR> {
        match (c, dir.to_owned()) {
            ('.', d) => vec![d],
            ('/', NORTH) => vec![EAST],
            ('/', EAST) => vec![NORTH],
            ('/', SOUTH) => vec![WEST],
            ('/', WEST) => vec![SOUTH],
            ('\\', NORTH) => vec![WEST],
            ('\\', EAST) => vec![SOUTH],
            ('\\', SOUTH) => vec![EAST],
            ('\\', WEST) => vec![NORTH],
            ('|', NORTH) => vec![NORTH],
            ('|', EAST) => vec![NORTH, SOUTH],
            ('|', SOUTH) => vec![SOUTH],
            ('|', WEST) => vec![NORTH, SOUTH],
            ('-', NORTH) => vec![EAST, WEST],
            ('-', EAST) => vec![EAST],
            ('-', SOUTH) => vec![EAST, WEST],
            ('-', WEST) => vec![WEST],
            _ => { panic!("Invalid combo!") }
        }
    }

    fn apply_p1(input: &str) -> usize {
        let grid = Grid::from_lines(input);

        run_grid(&grid, Point::new(0, 0), EAST)
    }

    fn apply_p2(input: &str) -> usize {
        let grid = Grid::from_lines(input);
        let (x_bounds, y_bounds) = grid.bounds();

        let mut starts: Vec<(Point, DIR)> = vec![];

        for x in x_bounds.to_owned() {
            starts.push((Point::new(x as i32, 0), SOUTH));
        }

        for x in x_bounds.to_owned() {
            starts.push((Point::new(x as i32, y_bounds.end as i32 - 1 ), NORTH));
        }

        for y in y_bounds.to_owned() {
            starts.push((Point::new(0, y as i32), EAST));
        }

        for y in y_bounds.to_owned() {
            starts.push((Point::new(x_bounds.end as i32 - 1, y as i32 ), WEST));
        }

        starts
            .into_iter()
            .map(|(p, d)| run_grid(&grid, p, d))
            .max()
            .unwrap()
    }

    fn run_grid(grid: &Grid, starting_point: Point, starting_dir: DIR) -> usize {
        let mut grid_energy: HashSet<(Point, DIR)> = HashSet::new();

        let mut queue: VecDeque<(Point, DIR)> = VecDeque::new();
        queue.push_front((starting_point, starting_dir));

        while !queue.is_empty() {
            let (p, dir) = queue.pop_front().unwrap();

            let c = grid.get_point(&p).unwrap();
            let new_dirs = interact(*c, &dir);

            for nd in new_dirs {
                let maybe_np = grid.try_move(&p, &nd);
                if maybe_np.is_some() {
                    let np = maybe_np.unwrap();

                    if !grid_energy.contains(&(np.to_owned(), nd)) {
                        queue.push_front((np.to_owned(), nd));
                    }
                }
            }

            grid_energy.insert((p, dir));
        }


        return grid_energy
            .iter()
            .map(|(p, _)| p)
            .collect::<HashSet<_>>()
            .len();
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
        let data = read_data_file(16, "input.txt");


        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(16, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}