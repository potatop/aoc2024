use std::fmt::Write;
use std::fmt::{Debug, Display, Formatter};
use std::io::BufRead;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Grid<T> {
    pub array: Vec<T>,
    pub height: usize,
    pub width: usize,
}
pub type Point = (usize, usize);

impl<T> Grid<T>
where
    T: Default + Copy,
{
    pub fn new(v: Vec<T>, h: usize, w: usize) -> Self {
        Self {
            array: v,
            height: h,
            width: w,
        }
    }
    pub fn from_reader<R: BufRead>(reader: R) -> Grid<u8> {
        let mut w = 0;
        let mut h = 0;
        let mut v = Vec::new();
        for s in reader.lines().map_while(Result::ok) {
            if w == 0 {
                w = s.len();
            } else {
                assert_eq!(w, s.len());
            }
            v.extend(s.bytes());
            h += 1;
        }
        Grid::new(v, h, w)
    }

    pub fn get(&self, yx: &Point) -> T {
        self.array[yx.0 * self.width + yx.1]
    }
    pub fn set(&mut self, yx: &Point, val: T) {
        self.array[yx.0 * self.width + yx.1] = val
    }

    pub fn map_index(&self, i: usize) -> Point {
        (i / self.width, i % self.width)
    }
}

impl<T> Display for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = (0..self.height).fold(String::new(), |mut output, i| {
            let _ = writeln!(
                output,
                "{:?}",
                &self.array[self.width * i..self.width * (i + 1)]
            );
            output
        });
        write!(f, "{}", s)
    }
}
