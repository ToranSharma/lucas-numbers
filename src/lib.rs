pub mod oeis;
pub use oeis::a000032 as lucas;
pub use oeis::a000045 as fibonacci;
pub use oeis::a000129 as pell;
pub use oeis::a002203 as pellLucas;
pub use oeis::a001045 as jacobsthal;
pub use oeis::a014551 as jacobsthalLucas;
pub use oeis::a000225 as mersenne;


pub enum SequenceType {
    U,
    V,
}

pub struct LucasSequence {
    p: i32,
    q: i32,
    current: i32,
    previous: i32,
}

impl LucasSequence {
    pub fn new(version: SequenceType, p: i32, q: i32) -> Self {
        match version {
            SequenceType::U => Self { p, q, previous: 0, current: 1, },
            SequenceType::V => Self { p, q,  previous: 2, current: p, }
        }
    }
    pub fn u(p: i32, q: i32) -> Self {
      Self::new(SequenceType::U, p, q)
    }
    
    pub fn v(p: i32, q:i32) -> Self {
      Self::new(SequenceType::V, p, q)
    }
}

impl Iterator for LucasSequence {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.p * self.current - self.q * self.previous;
        self.previous = self.current;
        self.current = next;
        Some(next)
    }
}