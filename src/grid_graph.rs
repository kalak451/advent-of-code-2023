use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::grid::{DIR, Grid, Point};
use crate::grid::DIR::{EAST, NORTH, SOUTH, WEST};

#[derive(Debug, Clone)]
pub struct GridNodeEdge {
    pub dest: Point,
    pub dir: DIR,
    pub cost: i64,
}

#[derive(Debug, Clone)]
pub struct GridNode {
    point: Point,
    symbol: char,
    edges: HashMap<Point, GridNodeEdge>,
}

impl GridNode {
    fn replace_edge(&mut self, to_replace: &Point, new_edge: GridNodeEdge) {
        self.edges.remove(to_replace);
        self.edges.insert(new_edge.dest, new_edge);
    }

    pub fn get_edges(&self) -> &HashMap<Point, GridNodeEdge> {
        return &self.edges;
    }
}

#[derive(Debug, Clone)]
pub struct GridNodeGraph {
    graph: HashMap<Point, GridNode>,
}

impl GridNodeGraph {
    pub fn from_grid(grid: &Grid, excluded_symbols: &HashSet<char>) -> GridNodeGraph {
        let mut graph = HashMap::new();

        for x in 0..grid.x_size {
            for y in 0..grid.y_size {
                let p = Point::new(x as i32, y as i32);
                if let Some(&c) = grid.get_point(&p) {
                    if !excluded_symbols.contains(&c) {
                        let mut edges = HashMap::new();

                        for d in [NORTH, SOUTH, EAST, WEST] {
                            if let Some(ap) = grid.try_move(&p, &d) {
                                if let Some(&ac) = grid.get_point(&ap) {
                                    if !excluded_symbols.contains(&ac) {
                                        edges.insert(ap, GridNodeEdge {
                                            dest: ap,
                                            dir: d,
                                            cost: 1,
                                        });
                                    }
                                }
                            }
                        }

                        let n = GridNode {
                            point: p,
                            symbol: c,
                            edges,
                        };

                        graph.insert(n.point, n);
                    }
                }
            }
        }

        return GridNodeGraph {
            graph
        };
    }

    fn simplify_once(&self, excluded_symbols: &HashSet<char>) -> (GridNodeGraph, usize) {
        let mut nodes_cut = 0;
        let mut new_graph = self.graph.clone();

        for key in self.graph.keys() {
            let node = new_graph.get(key).unwrap();

            if node.edges.len() != 2 {
                continue;
            }

            if excluded_symbols.contains(&node.symbol) {
                continue;
            }

            let (e1, e2) = node.edges.values().collect_tuple().unwrap();
            let new_cost = e1.cost + e2.cost;
            let e1_dest = e1.dest;
            let e2_dest = e1.dest;

            let new_e1ne = GridNodeEdge {
                dest: e2_dest,
                cost: new_cost,
                dir: e2.dir,
            };

            let new_e2ne = GridNodeEdge {
                dest: e1_dest,
                cost: new_cost,
                dir: e1.dir,
            };

            let e1n = new_graph.get_mut(&e1_dest).unwrap();
            e1n.replace_edge(key, new_e1ne);

            let e2n = new_graph.get_mut(&e2_dest).unwrap();
            e2n.replace_edge(key, new_e2ne);

            new_graph.remove(key);
            nodes_cut += 1;
        }

        return (
            GridNodeGraph {
                graph: new_graph
            },
            nodes_cut
        );
    }

    pub fn simplify(&self, excluded_symbols: &HashSet<char>) -> (GridNodeGraph, usize) {

        let mut all_cut = 0;
        let (mut working_graph, mut cnt) = self.simplify_once(excluded_symbols);

        loop {
            if cnt == 0 {
                break;
            }
            all_cut += cnt;

            (working_graph, cnt) = working_graph.simplify_once(excluded_symbols);
        }


        return (working_graph, all_cut);
    }

    pub fn get(&self, p: &Point) -> Option<&GridNode> {
        return self.graph.get(p);
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        return self.graph.contains_key(p);
    }
}

