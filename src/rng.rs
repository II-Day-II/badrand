use std::{ops::{Range}};

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn seed(&mut self, state: u64) {
        self.state = state;
    }
    pub fn rnd(&mut self) -> u64 {
        self.state += 0x60bee2bee120fc15;
        let mut tmp: u128 = self.state as u128 * 0xa3b195354a39b70d;
        let m1 = ((tmp >> 64) ^ tmp) as u64;
        tmp = m1 as u128 * 0x1b03738712fad5c9;
        let m2 = ((tmp >> 64) ^ tmp) as u64;
        m2 
    }
    pub fn range_u64(&mut self, range: Range<u64>) -> u64 {
        self.rnd() % (range.end - range.start) + range.start
    } 
    pub fn range_u32(&mut self, range: Range<u32>) -> u32 {
        self.range_u64(range.start as u64..range.end as u64) as u32
    }
    pub fn range_f64(&mut self, range: Range<f64>) -> f64 {
        (self.rnd() as f64 / (0x7FFFFFFFFFFFFFFFu64 as f64)) * (range.end - range.start) + range.start
    }
    pub fn range_f32(&mut self, range: Range<f32>) -> f32 {
        self.range_f64(range.start as f64..range.end as f64) as f32
    }
}