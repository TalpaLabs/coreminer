use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::Word;

pub type RawPointer = *mut std::ffi::c_void;

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addr(usize);

impl Addr {
    pub fn usize(&self) -> usize {
        self.0
    }
    pub fn u64(&self) -> u64 {
        self.0 as u64
    }
    pub fn raw_pointer(&self) -> RawPointer {
        self.0 as RawPointer
    }
}

impl Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", { self.0 })
    }
}

impl Add for Addr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<usize> for Addr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign for Addr {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl AddAssign<usize> for Addr {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

impl SubAssign for Addr {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl SubAssign<usize> for Addr {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs
    }
}

impl Sub for Addr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<usize> for Addr {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl From<RawPointer> for Addr {
    fn from(value: RawPointer) -> Self {
        Addr(value as usize)
    }
}

impl From<Addr> for RawPointer {
    fn from(value: Addr) -> Self {
        value.0 as RawPointer
    }
}

impl From<usize> for Addr {
    fn from(value: usize) -> Self {
        Addr(value)
    }
}

impl From<Word> for Addr {
    fn from(value: Word) -> Self {
        Addr(value as usize)
    }
}

impl From<u64> for Addr {
    fn from(value: u64) -> Self {
        Addr(value as usize)
    }
}

impl From<Addr> for Word {
    fn from(value: Addr) -> Self {
        value.0 as Word
    }
}

impl From<Addr> for u64 {
    fn from(value: Addr) -> Self {
        value.0 as u64
    }
}

impl std::fmt::Debug for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addr_arithmetic() {
        let a = Addr::from(100usize);
        let b = Addr::from(50usize);
        assert_eq!((a + b).usize(), 150);
        assert_eq!((a - b).usize(), 50);
    }

    #[test]
    fn test_addr_conversions() {
        let a = Addr::from(0x1234usize);
        assert_eq!(a.u64(), 0x1234u64);
        assert_eq!(format!("{}", a), "0x0000000000001234");
    }
}
