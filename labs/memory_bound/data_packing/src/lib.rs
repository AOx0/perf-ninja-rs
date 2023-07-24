#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

use rand::prelude::*;

use std::cmp::{Ord, Ordering, PartialOrd};

#[cfg(test)]
mod tests;

mod init;
pub use init::{create_entry, init};

// Assume those constants never change
pub const N: usize = 10_000;
pub const MIN_RANDOM: u8 = 0;
pub const MAX_RANDOM: u8 = 100;

// FIXME: this data structure can be reduced in size
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct S {
    pub i: u8,
    /// The last bit is the b flag
    /// ```
    /// b_iii_iiii     
    /// ```
    pub s: u8,
    pub l: i16,
    pub d: f32,
}

impl S {
    fn set_b(&mut self, value: bool) {
        self.s = self.s & !0b1_000_0000 | (value as u8) << 7;
    }

    fn get_b(&self) -> bool {
        self.s >> 7 == 1
    }

    fn get_s(&self) -> u8 {
        self.s & !(0b1_000_0000)
    }

    fn set_s(&mut self, value: u8) {
        self.s = self.s & 0b1_000_0000 | value;
    }
}

// C++ version overloads '<' operator like this:
//   bool operator<(const S &s) const { return this->i < s.i; }
// I presume that means we order / sort / compare based only on value of 'i'?

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.i.partial_cmp(&other.i)
    }
}
impl Ord for S {
    fn cmp(&self, other: &Self) -> Ordering {
        self.i.cmp(&other.i)
    }
}
impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}
impl Eq for S {}

pub fn solution(arr: &mut [S]) {
    // 1. shuffle
    let mut rd = thread_rng();
    arr.shuffle(&mut rd);

    // 2. sort
    arr.sort_unstable();
}
