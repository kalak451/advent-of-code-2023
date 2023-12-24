#[cfg(test)]
mod day23 {
    use std::cmp;
    use std::collections::{HashMap, HashSet, VecDeque};

    use itertools::Itertools;

    use crate::grid::{Grid, Point};
    use crate::grid::DIR::{EAST, NORTH, SOUTH, WEST};
    use crate::grid_graph::GridNodeGraph;
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[derive(Debug, Clone)]
    struct Node {
        name: Point,
        sym: char,
        edges: Vec<(Point, i64)>,
    }

    fn build_graph(grid: &Grid) -> HashMap<Point, Node> {
        let mut results = HashMap::new();

        for x in 0..grid.x_size {
            for y in 0..grid.y_size {
                let p = Point::new(x as i32, y as i32);
                if let Some(&c) = grid.get_point(&p) {
                    if c != '#' {
                        let adj_points = adj_p2(grid, &p);
                        let edges = adj_points.into_iter().map(|a| (a, 1)).collect_vec();

                        let n = Node {
                            name: p,
                            sym: c,
                            edges,
                        };

                        results.insert(p, n);
                    }
                }
            }
        }

        return results;
    }

    fn trim_graph(graph: &HashMap<Point, Node>) -> (HashMap<Point, Node>, usize) {
        let mut new_graph = graph.clone();
        let mut nodes_cut = 0;

        for (k, _) in graph {
            let node = new_graph.get(k).unwrap();
            if node.edges.len() == 2 {
                let (e1, e2) = node.edges.iter().collect_tuple().unwrap();

                let new_cost: i64 = e1.1 + e2.1;

                let e1n = new_graph.get(&e1.0).unwrap();
                let e2n = new_graph.get(&e2.0).unwrap();

                let new_e1n = Node {
                    name: e1n.name,
                    sym: e1n.sym,
                    edges: e1n.edges.iter().map(|(ep, ec)| {
                        if *ep == *k {
                            (e2n.name, new_cost)
                        } else {
                            (*ep, *ec)
                        }
                    }).collect_vec(),
                };

                let new_e2n = Node {
                    name: e2n.name,
                    sym: e2n.sym,
                    edges: e2n.edges.iter().map(|(ep, ec)| {
                        if *ep == *k {
                            (e1n.name, new_cost)
                        } else {
                            (*ep, *ec)
                        }
                    }).collect_vec(),
                };

                new_graph.insert(new_e1n.name, new_e1n);
                new_graph.insert(new_e2n.name, new_e2n);
                new_graph.remove(&k);
                nodes_cut += 1;
            }
        }

        return (new_graph, nodes_cut);
    }

    fn prep_graph(grid: &Grid) -> GridNodeGraph {
        let (graph, all_cut) = GridNodeGraph::from_grid(grid, &HashSet::from_iter(['#']))
            .simplify(&HashSet::new());

        println!("Cut {all_cut} nodes");

        return graph;
    }

    fn longest_path_graph(graph: &GridNodeGraph, start: Point, end: Point, adj: fn(&GridNodeGraph, &Point) -> Vec<(Point, i64)>) -> i64 {
        let mut stk = VecDeque::new();
        stk.push_front((start, 0i64, HashSet::new()));

        let mut max_path_length = 0i64;

        while let Some((p, total_cost, mut seen)) = stk.pop_front() {
            seen.insert(p);

            if p == end {
                max_path_length = cmp::max(max_path_length, total_cost);
            } else {
                for (a, cost) in adj(graph, &p) {
                    if !seen.contains(&a) {
                        stk.push_front((a, total_cost + cost, seen.clone()));
                    }
                }
            }
        }

        return max_path_length;
    }

    fn graph_adj_p2(graph: &GridNodeGraph, p: &Point) -> Vec<(Point, i64)> {
        let node = graph.get(p).unwrap();
        return node.get_edges().values().map(|e| (e.dest, e.cost)).collect_vec();
    }

    fn apply_p1(input: &str) -> i64 {
        let grid = Grid::from_lines(input);
        let start = Point::new(1, 0);
        let end = Point::new((grid.x_size - 2) as i32, (grid.y_size - 1) as i32);

        return longest_path(&grid, start, end, adj) - 1;
    }

    fn apply_p2(input: &str) -> i64 {
        let grid = Grid::from_lines(input);
        let start = Point::new(1, 0);
        let end = Point::new((grid.x_size - 2) as i32, (grid.y_size - 1) as i32);
        let graph = prep_graph(&grid);

        assert!(graph.contains_point(&start));
        assert!(graph.contains_point(&end));

        return longest_path_graph(&graph, start, end, graph_adj_p2);
    }


    fn longest_path(grid: &Grid, start: Point, end: Point, adj: fn(&Grid, &Point) -> Vec<Point>) -> i64 {
        let mut stk = VecDeque::new();
        stk.push_front((start, HashSet::new()));

        let mut max_path_length = 0i64;

        while let Some((p, mut seen)) = stk.pop_front() {
            seen.insert(p);

            if p == end {
                max_path_length = cmp::max(max_path_length, seen.len() as i64);
            } else {
                for a in adj(grid, &p) {
                    if !seen.contains(&a) {
                        stk.push_front((a, seen.clone()));
                    }
                }
            }
        }

        return max_path_length;
    }

    fn adj(grid: &Grid, p: &Point) -> Vec<Point> {
        let mut results = vec![];
        for d in [NORTH, SOUTH, EAST, WEST] {
            if let Some(np) = grid.try_move(p, &d) {
                match grid.get_point(&np) {
                    Some('.') => { results.push(np); }
                    Some('^') if d == NORTH => { results.push(np); }
                    Some('v') if d == SOUTH => { results.push(np); }
                    Some('<') if d == WEST => { results.push(np); }
                    Some('>') if d == EAST => { results.push(np); }
                    _ => {}
                }
            }
        }

        return results;
    }

    fn adj_p2(grid: &Grid, p: &Point) -> Vec<Point> {
        let mut results = vec![];
        for d in [NORTH, SOUTH, EAST, WEST] {
            if let Some(np) = grid.try_move(p, &d) {
                match grid.get_point(&np) {
                    Some('.') => { results.push(np); }
                    Some('^') => { results.push(np); }
                    Some('v') => { results.push(np); }
                    Some('<') => { results.push(np); }
                    Some('>') => { results.push(np); }
                    _ => {}
                }
            }
        }

        return results;
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
        let data = read_data_file(23, "input.txt");


        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(23, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}