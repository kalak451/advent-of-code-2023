use std::ops::Range;
use std::slice::SliceIndex;

pub struct Grid {
    data: Vec<Vec<char>>,
    x_size: usize,
    y_size: usize
}

impl Grid {
    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        return self.data.get(y).map(|r| r.get(x)).flatten();
    }

    pub fn get_x_slice<R: SliceIndex<[char], Output = [char]>>(&self, x_range: R, y: usize) -> Option<&[char]> {
        return self.data.get(y).map(|r| r.get(x_range)).flatten();
    }

    pub fn bounds(&self) -> (Range<usize>, Range<usize>) {
        return (
            0..self.x_size,
            0..self.y_size
            );
    }

    pub fn pos_is(&self, x: usize, y: usize, pred: fn(Option<&char>) -> bool) -> bool {
        pred(self.get(x,y))
    }

    pub fn pos_is_ascii_digit(&self, x: usize, y: usize) -> bool {
        self.pos_is(x,y, |o| o.map(|c| c.is_ascii_digit()).unwrap_or(false))
    }

    pub fn is_x_bound(&self, x: usize) -> bool {
        return x == 0 || x == self.x_size - 1;
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
            y_size
        };
    }
}