use std::{
    cmp::{max, min},
    fmt::Display,
    ops::{Add, Range, Sub},
};

#[derive(Eq, PartialEq, PartialOrd, Ord, Default, Hash, Clone, Copy, Debug)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

#[derive(Debug)]
pub struct Bounds(pub Pos, pub Pos);

impl Bounds {
    pub fn from_iter<'a>(mut positions: impl Iterator<Item = &'a Pos>) -> Option<Bounds> {
        if let Some(fst) = positions.next() {
            let mut max_x = fst.x;
            let mut min_x = fst.x;

            let mut max_y = fst.y;
            let mut min_y = fst.y;

            for p in positions {
                max_x = max(max_x, p.x);
                min_x = min(min_x, p.x);

                max_y = max(max_y, p.y);
                min_y = min(min_y, p.y);
            }

            return Some(Bounds(Pos::new(min_x, min_y), Pos::new(max_x, max_y)));
        }
        None
    }

    pub fn expand(&self) -> Bounds {
        Bounds(self.0.up().left(), self.1.down().right())
    }

    pub fn along_x(&self) -> Range<i64> {
        self.0.x..self.1.x
    }

    pub fn along_y(&self) -> Range<i64> {
        self.0.y..self.1.y
    }
}

impl Pos {
    pub fn north() -> Self {
        Pos { x: 0, y: -1 }
    }

    pub fn south() -> Self {
        Pos { x: 0, y: 1 }
    }

    pub fn west() -> Self {
        Pos { x: -1, y: 0 }
    }
    pub fn east() -> Self {
        Pos { x: 1, y: 0 }
    }

    pub fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }

    pub fn cross(&self) -> impl Iterator<Item = Pos> {
        vec![self.up(), self.down(), self.right(), self.left()].into_iter()
    }

    pub fn up(&self) -> Pos {
        Pos {
            y: self.y - 1,
            x: self.x,
        }
    }
    pub fn down(&self) -> Pos {
        Pos {
            y: self.y + 1,
            x: self.x,
        }
    }
    pub fn left(&self) -> Pos {
        Pos {
            y: self.y,
            x: self.x - 1,
        }
    }
    pub fn right(&self) -> Pos {
        Pos {
            y: self.y,
            x: self.x + 1,
        }
    }

    pub fn from_linear(i: i64, width: i64) -> Pos {
        Pos {
            y: i / width,
            x: i % width,
        }
    }

    pub fn to_linear(&self, width: i64) -> usize {
        (self.y * width + self.x) as usize
    }

    pub fn gcd_vec(&self) -> Self {
        let d = gcd(self.x, self.y);
        Pos {
            x: self.x / d,
            y: self.y / d,
        }
    }

    pub fn within_bounds(&self, bounds: &Pos) -> bool {
        self.x >= 0 && self.x < bounds.x && self.y >= 0 && self.y < bounds.y
    }
}
