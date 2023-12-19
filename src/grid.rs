use std::ops::Range;
use std::slice::SliceIndex;
use itertools::Itertools;
use crate::grid::DIR::{EAST, NORTH, SOUTH, WEST};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Point {
            x: x as usize,
            y: y as usize,
        };
    }

    pub fn apply_dir_vector(&self, vec: &Vec<i32>) -> Option<Point> {
        let new_x = self.x as i32 + vec[0];
        let new_y = self.y as i32 + vec[1];

        if new_x >= 0 && new_y >= 0 {
            return Some(Point::new(new_x, new_y));
        }

        return None;
    }
}


pub struct Grid {
    data: Vec<Vec<char>>,
    pub x_size: usize,
    pub y_size: usize,
}

impl Grid {
    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        return self.data.get(y).map(|r| r.get(x)).flatten();
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        self.data[y][x] = c;
    }

    // pub fn set_point(&mut self, p: &Point, c: char) {
    //     self.set(p.x, p.y, c);
    // }

    pub fn get_point(&self, point: &Point) -> Option<&char> {
        return self.data.get(point.y).map(|r| r.get(point.x)).flatten();
    }

    pub fn get_x_slice<R: SliceIndex<[char], Output=[char]>>(&self, x_range: R, y: usize) -> Option<&[char]> {
        return self.data.get(y).map(|r| r.get(x_range)).flatten();
    }

    pub fn bounds(&self) -> (Range<usize>, Range<usize>) {
        return (
            0..self.x_size,
            0..self.y_size
        );
    }

    pub fn pos_is(&self, x: usize, y: usize, pred: fn(Option<&char>) -> bool) -> bool {
        pred(self.get(x, y))
    }

    pub fn pos_is_ascii_digit(&self, x: usize, y: usize) -> bool {
        self.pos_is(x, y, |o| o.map(|c| c.is_ascii_digit()).unwrap_or(false))
    }

    pub fn is_x_bound(&self, x: usize) -> bool {
        return x == 0 || x == self.x_size - 1;
    }

    pub fn get_points(&self, from: &Point, in_dir_of: &DIR) -> Vec<Point> {
        if *in_dir_of == NORTH && from.y == 0 {
            return vec![];
        }

        if *in_dir_of == SOUTH && from.y == self.y_size - 1 {
            return vec![];
        }

        if *in_dir_of == WEST && from.x == 0 {
            return vec![];
        }

        if *in_dir_of == EAST && from.x == self.x_size - 1 {
            return vec![];
        }

        let (xr, yr) = match in_dir_of {
            EAST => (from.x + 1..self.x_size, from.y..from.y + 1),
            WEST => (0..from.x, from.y..from.y + 1),
            SOUTH => (from.x..from.x + 1, from.y + 1..self.y_size),
            NORTH => (from.x..from.x + 1, 0..from.y)
        };

        let mut result = vec![];

        for y in yr.to_owned() {
            for x in xr.to_owned() {
                result.push(Point::new(x as i32, y as i32));
            }
        }

        if *in_dir_of == WEST || *in_dir_of == NORTH {
            result.reverse();
        }

        return result;
    }

    pub fn try_move(&self, p: &Point, dir: &DIR) -> Option<Point> {
        match dir {
            NORTH => {
                if p.y == 0 {
                    None
                } else {
                    Some(Point::new(p.x as i32, p.y as i32 - 1))
                }
            }
            SOUTH => {
                if p.y == self.y_size - 1 {
                    None
                } else {
                    Some(Point::new(p.x as i32, p.y as i32 + 1))
                }
            }
            EAST => {
                if p.x == self.x_size - 1 {
                    None
                } else {
                    Some(Point::new(p.x as i32 + 1, p.y as i32))
                }
            }
            WEST => {
                if p.x == 0 {
                    None
                } else {
                    Some(Point::new(p.x as i32 - 1, p.y as i32))
                }
            }
        }
    }

    pub fn print(&self) {
        for r in &self.data {
            let l = r.into_iter().join("");
            println!("{l}")
        }
    }

    pub fn print_string(&self) -> String {
        self.data.iter()
            .map(|r| r.into_iter().join(""))
            .join("\n")
    }

    pub fn from_lines(s: &str) -> Grid {
        let data = s
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let x_size = data[0].len();
        let y_size = data.len();

        return Grid {
            data,
            x_size,
            y_size,
        };
    }

    // pub fn from_size(x_size: usize, y_size: usize) -> Grid {
    //     let data = vec![vec!['.'; x_size]; y_size];
    //
    //     return Grid {
    //         data,
    //         x_size,
    //         y_size
    //     };
    // }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Copy, Ord, PartialOrd)]
pub enum DIR {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}