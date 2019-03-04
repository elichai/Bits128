#![no_std]

//! # bits128
//! `bits128` provides a struct that let's you use 128 bits while taking only 128 bits in memory. <br>
//! if you would use something like `[bool; 128]` it would take 128*8 bits in memory because every bool takes 1 bytes(8bits) <br>
//! In the future I'll implement an Iterator over the bits so you can iterate over them easily. <br>


use core::{fmt, ops};

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Ord, Default, Hash)]
pub struct Bits128(u128);

impl Bits128 {
    const FIRST: u128 = 1 << 127;
    const BITS: [bool;2] = [false, true];

    /// This creates the struct with all bits set to zero
    pub fn empty() -> Self {
        Self(0)
    }

    /// Creates a struct using a u128 integer.
    /// ## Examples
    /// ```rust
    /// # use bits128::Bits128;
    /// let bits = Bits128::from_dec(1024);
    /// println!("{}", bits) // will print `[1,0,0,0,0,0,0,0,0,0,0]`
    /// ```
    pub fn from_dec(dec: u128) -> Self {
        Self(dec)
    }

    /// Creates a struct using a u128 integer.
    /// ## Examples
    /// ```rust
    /// # use bits128::Bits128;
    /// let bits = Bits128::from_dec(1025);
    /// assert!(bits.last());
    /// ```
    pub fn last(&self) -> bool {
        self.0 & 1 == 1
    }

    /// Creates a struct using a u128 integer.
    /// ## Examples
    /// ```rust
    /// # use bits128::Bits128;
    /// let bits = Bits128::from_dec(2*(2u128.pow(127)-1));
    /// assert!(bits.first());
    /// ```
    pub fn first(&self) -> bool {
        self.0 & Self::FIRST == Self::FIRST
    }

    /// Creates a struct using a u128 integer.
    /// ## Examples
    /// ```rust
    /// # use bits128::Bits128;
    /// let bits = Bits128::from_dec(5);
    /// assert!(bits.at(2));
    /// ```
    pub fn at(&self, location: usize) -> bool {
        self.0 >> location & 1 == 1
    }

    /// Creates a struct using a u128 integer.
    /// ## Examples
    /// ```rust
    /// # use bits128::Bits128;
    /// let mut bits = Bits128::empty();
    /// assert!(!bits.at(10));
    /// bits.flip(10);
    /// assert!(bits.at(10));
    /// ```
    pub fn flip(&mut self, location: usize)  {
        let xor = 1 << location;
        self.0 ^= xor;
    }

    /// Creates a struct using a u128 integer.
    /// ## Examples
    /// ```rust
    /// # use bits128::Bits128;
    /// let bits = Bits128::from_dec(512);
    /// assert_eq!(bits.len(), 9);
    /// ```
    pub fn len(&self) -> usize {
        let mut s = *self;
        for i in (0..128).rev() {
            if s.first() {
                return i;
            }
            s.0 =  s.0 << 1;
        }
        0
    }
}

impl ops::Index<usize> for Bits128 {
    type Output = bool;
    fn index(&self, location: usize) -> &bool {
        &Self::BITS[self.at(location) as usize]
    }
}


impl fmt::Display for Bits128 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let significant = self.len();
        let mut s = self.0;
        let location = 1 << significant;
        write!(f, "[")?;
        for _ in 0..=significant {
            write!(f, "{},", (s | location == s) as u8)?;
            s = s << 1;
        }
        write!(f, "]")
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Write a macro to test a bunch of different integers/
    #[test]
    fn test_at() {
        let a = Bits128(117);
        assert!(a.at(0));
        assert!(!a.at(1));
        assert!(a.at(2));
        assert!(!a.at(3));
        assert!(a.at(4));
        assert!(a.at(5));
        assert!(a.at(6));
        let a = Bits128(2560);
        assert!(!a.at(0));
        assert!(!a.at(1));
        assert!(!a.at(2));
        assert!(!a.at(3));
        assert!(!a.at(4));
        assert!(!a.at(5));
        assert!(!a.at(6));
        assert!(!a.at(7));
        assert!(!a.at(8));
        assert!(a.at(9));
        assert!(!a.at(0));
        assert!(a.at(11));
    }
    #[test]
    fn test_flip() {
        let mut a = Bits128(2403923381);
        a.flip(29);
        assert_eq!(a.0, 2940794293);
        let mut a = Bits128(9220072036854775807);
        a.flip(42);
        assert_eq!(a.0, 9220067638808264703);
    }
    #[test]
    fn test_last() {
        assert!(Bits128(55).last());
        assert!(Bits128(45678987621).last());
        assert!(Bits128(657483957483957433).last());
    }
}

//100
//110
//101
//10001



// TODO: Benchmark AND vs. MODULO 2.