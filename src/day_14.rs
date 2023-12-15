#[cfg(test)]
mod day14 {
    use itertools::Itertools;
    use crate::grid::{DIR, Grid, Point};
    use crate::grid::DIR::{EAST, NORTH, SOUTH, WEST};
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    fn lean_grid(grid: &mut Grid, dir: &DIR) {
        let (x_bounds, y_bounds) = grid.bounds();

        let x_range = if *dir == EAST {
            x_bounds.to_owned().rev().collect_vec()
        } else {
            x_bounds.to_owned().collect_vec()
        };

        let y_range = if *dir == SOUTH {
            y_bounds.to_owned().rev().collect_vec()
        } else {
            y_bounds.to_owned().collect_vec()
        };

        for y in &y_range {
            for x in &x_range {
                let point = Point::new(*x as i32, *y as i32);
                let c = grid.get_point(&point).unwrap();
                if *c == 'O' {
                    let mut final_move: Option<Point> = None;
                    let possible_moves = grid.get_points(&point, &dir);
                    for m in possible_moves {
                        let mv = grid.get_point(&m).unwrap();
                        if *mv == '.' {
                            final_move = Some(m);
                        } else {
                            break;
                        }
                    }

                    if final_move.is_some() {
                        let mm = final_move.unwrap();
                        grid.set(*x, *y, '.');
                        grid.set(mm.x, mm.y, 'O');
                    }
                }
            }
        }
    }

    fn calculate_load(grid: &Grid, dir: &DIR) -> usize {
        let (start_x_cost, dx, start_y_cost, dy) = match dir {
            NORTH => (0, 0, grid.y_size as i32, -1),
            SOUTH => (0, 0, 0, 1),
            EAST => (0, 1, 0, 0),
            WEST => (grid.x_size as i32 - 1, -1, 0, 0)
        };

        let (x_bounds, y_bounds) = grid.bounds();

        let mut total = 0;
        let mut y_cost = start_y_cost;
        for y in y_bounds.to_owned() {
            let mut x_cost = start_x_cost;
            for x in x_bounds.to_owned() {
                let c = grid.get(x, y).unwrap();
                if *c == 'O' {
                    total = total + x_cost + y_cost;
                }
                x_cost = x_cost + dx;
            }
            y_cost = y_cost + dy;
        }

        return total as usize;
    }

    fn apply_p1(grid: &mut Grid) -> usize {
        lean_grid(grid, &NORTH);
        grid.print();

        return calculate_load(grid, &NORTH);
    }

    fn run_cycle(grid: &mut Grid) {
        lean_grid(grid, &NORTH);
        lean_grid(grid, &WEST);
        lean_grid(grid, &SOUTH);
        lean_grid(grid, &EAST);
    }

    #[test]
    fn sample_1_p1() {
        let data = SAMPLE_1;

        let mut grid = Grid::from_lines(data);
        let results = apply_p1(&mut grid);
        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_1_p2() {
        let data = SAMPLE_1;
        let mut grid = Grid::from_lines(&data);



        let results = apply_p2(&mut grid);
        println!("Answer: {results:?}");
    }

    fn apply_p2(mut grid: &mut Grid) -> usize {
        let (offset, cycle_size, storage) = find_cycle(&mut grid).unwrap();

        let total_cycles = 1000000000usize;

        let remainder = (total_cycles - offset) % cycle_size;

        let x = &storage[offset - 1 + remainder];
        let new_grid = Grid::from_lines(x);

        return calculate_load(&new_grid, &NORTH);
    }

    fn find_cycle(mut grid: &mut &mut Grid) -> Option<(usize, usize, Vec<String>)> {
        let mut storage: Vec<String> = vec![];

        for i in 0..1000 {
            run_cycle(&mut grid);
            let cycle_res = grid.print_string();

            let pos = storage.iter().position(|v| *v == cycle_res);
            if pos.is_some() {
                let first = pos.unwrap();
                let dif = i - first;
                println!("first: {first}, i: {i}, diff: {dif}");
                return Some((first + 1, dif, storage))
            }

            storage.push(cycle_res);
        }

        return None;
    }

    #[test]
    fn part_1() {
        let data = read_data_file(14, "input.txt");

        let mut grid = Grid::from_lines(&data);
        let results = apply_p1(&mut grid);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(14, "input.txt");
        let mut grid = Grid::from_lines(&data);


        let results = apply_p2(&mut grid);
        println!("Answer: {results:?}");
    }
}