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

    pub fn get(&self, y: usize, x: usize) -> u8 {
        self.array[y * self.height + x]
    }

    pub fn get_at(&self, yx: Point) -> u8 {
        self.get(yx.0, yx.1)
    }
}
