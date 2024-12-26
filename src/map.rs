use std::ops::{Index, IndexMut};

use crate::pos::Pos;

#[derive(Debug)]
pub struct Map<T> {
    data: Vec<T>,
    width: i64,
    height: i64,
}

impl From<&str> for Map<u8> {
    fn from(value: &str) -> Self {
        assert!(!value.is_empty(), "map input must be >0");
        let mut lines = value.lines();
        let first_line = lines.next().unwrap();
        let width = first_line.len();

        let mut data = Vec::from(first_line.as_bytes());

        let mut height = 1;
        for ln in lines {
            height += 1;
            data.extend_from_slice(ln.as_bytes());
            assert_eq!(width, ln.len());
        }

        Self {
            data,
            width: width as i64,
            height: height as i64,
        }
    }
}

impl<T> Index<&Pos> for Map<T> {
    type Output = T;

    fn index(&self, index: &Pos) -> &Self::Output {
        &self.data[index.to_linear(self.width)]
    }
}

impl<T> IndexMut<&Pos> for Map<T> {
    fn index_mut(&mut self, index: &Pos) -> &mut Self::Output {
        &mut self.data[index.to_linear(self.width)]
    }
}

impl Map<u8> {
    pub fn draw(&self) {
        for row in 0..self.height {
            println!(
                "{}",
                std::str::from_utf8(
                    &self.data[(row * self.width) as usize..((row + 1) * self.width) as usize]
                )
                .expect("map draw utf8")
            );
        }
    }
}

impl<T> Map<T> {
    pub fn bounds(&self) -> Pos {
        Pos::new(self.width, self.height)
    }

    pub fn new_with_size(width: i64, height: i64, cell: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![cell; (height * width) as usize],
            width,
            height,
        }
    }

    pub fn new_with_size_as<U>(other: &Map<U>, cell: T) -> Self
    where
        T: Clone,
    {
        Map::new_with_size(other.width, other.height, cell)
    }

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Map<U> {
        Map {
            data: Vec::from_iter(self.data.iter().map(f)),
            width: self.width,
            height: self.height,
        }
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        self.data.get(pos.to_linear(self.width))
    }

    pub fn set(&mut self, x: i64, y: i64, val: T) {
        self.data[(y * self.width + x) as usize] = val;
    }

    pub fn position<U: Fn(&T) -> bool>(&self, f: U) -> Option<Pos> {
        self.data
            .iter()
            .position(f)
            .map(|i| Pos::from_linear(i as i64, self.width))
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> {
        (0..self.width * self.height).map(|i| Pos::from_linear(i, self.width))
    }
}
