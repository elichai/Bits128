
#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Ord, Default, Hash)]
struct Bits128(u128);

impl Bits128 {
    const FIRST: u128 = 2^128;
    pub fn last(&self) -> bool {
        self.0 & 1 == 1
    }
    pub fn first(&self) -> bool {
        self.0 & Self::FIRST == 1
    }

    pub fn at(&self, location: usize) -> bool {
        let or = 2u128.pow(location as u32);
        self.0 | or == self.0
    }

    pub fn flip(&mut self, location: usize)  {
        let xor = 2u128.pow(location as u32);
        self.0 ^= xor;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let bits = Bits128(5);
        dbg!(17|4);
        dbg!(bits.last());
    }
    #[test]
    fn test_at() {
        let a = Bits128(117);
        dbg!(a.at(0));
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