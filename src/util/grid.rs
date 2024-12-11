use std::fmt::{Display, Formatter};
use std::io::BufRead;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Grid {
    pub array: Vec<u8>,
    pub height: usize,
    pub width: usize,
}
pub type Point = (usize, usize);

impl Grid {
    pub fn new<R: BufRead>(reader: R) -> Self {
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
        Grid {
            array: v,
            height: h,
            width: w,
        }
    }

    pub fn get(&self, yx: &Point) -> u8 {
        self.array[yx.0 * self.height + yx.1]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(
                (0..self.height)
                    .map(|i| &self.array[self.height * i..self.height * i + self.width]),
            )
            .finish()
    }
}
