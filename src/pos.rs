use std::ops::{Add, Sub};

#[derive(Eq, PartialEq, PartialOrd, Default, Hash, Clone, Copy, Debug)]
pub struct Pos {
    pub y: i32,
    pub x: i32,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

impl Pos {
    pub fn new(y: i32, x: i32) -> Self {
        Pos { y, x }
    }

    pub fn from_linear(i: i32, width: i32) -> Pos {
        Pos {
            y: i / width,
            x: i % width,
        }
    }

    pub fn to_linear(&self, width: i32) -> usize {
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
