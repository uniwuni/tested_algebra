#[tested_trait::tested_trait]
pub trait Setoid where Self: Clone {
    fn equiv(self: &Self, b: &Self) -> bool;
    fn prop_equiv_reflexive(a: Self) -> bool {
        *(&a.equiv(&a))
    }
    fn prop_equiv_symmetric(a : Self, b: Self) -> bool {
        *(&a.equiv(&b)) == *(&b.equiv(&a))
    }
    fn prop_equiv_transitive(a : Self, b: Self, c: Self) -> bool { // de facto uncheckable with default Arbitrary
        if *(&a.equiv(&b)) && *(&b.equiv(&c)) {
            *(&a.equiv(&c))
        } else {
            true
        }
    }
    #[test]
    fn equiv_reflexive() where Self: quickcheck::Arbitrary + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_equiv_reflexive as fn(_: Self) -> bool);
    }
    #[test]
    fn equiv_symmetric() where Self: quickcheck::Arbitrary + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_equiv_symmetric as fn(_: Self, _:Self) -> bool);
    }
    #[test]
    fn equiv_transitive() where Self: quickcheck::Arbitrary + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_equiv_transitive as fn(_: Self, _:Self, _:Self) -> bool);
    }
}

#[tested_trait::tested_trait]
pub trait ArbitrarySetoid where Self: Setoid + quickcheck::Arbitrary {
    fn generate_equiv(&self, g: &mut quickcheck::Gen) -> Self;

    fn prop_equiv_generate_equiv(a : Self, size: u8) -> Result<(), Self> where Self: quickcheck::Arbitrary {
        let mut g = quickcheck::Gen::new(size as usize);
        let b = Self::generate_equiv(&a, &mut g);
        if *(&a.equiv(&b)) { Ok(()) } else {Err(b)}
    }

    fn prop_equiv_generate_equiv_symmetric(a : Self, size: u8) -> bool where Self: quickcheck::Arbitrary {
        let mut g = quickcheck::Gen::new(size as usize);
        let b = Self::generate_equiv(&a, &mut g);
        *(&b.equiv(&a))
    }
    fn prop_equiv_generate_equiv_transitive(a : Self, size: u8) -> bool where Self: quickcheck::Arbitrary {
        let mut g = quickcheck::Gen::new(size as usize);
        let b = Self::generate_equiv(&a, &mut g);
        let c = Self::generate_equiv(&b, &mut g);
        *(&a.equiv(&c))
    }
    #[test]
    fn equiv_generate_equiv() where Self: quickcheck::Arbitrary + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_equiv_generate_equiv as fn(_: Self, _:u8) -> Result<(),Self>);
    }
    #[test]
    fn equiv_generate_equiv_symmetric() where Self: quickcheck::Arbitrary + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_equiv_generate_equiv_symmetric as fn(_: Self, _:u8) -> bool);
    }
    #[test]
    fn equiv_generate_equiv_transitive() where Self: quickcheck::Arbitrary + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_equiv_generate_equiv_transitive as fn(_: Self, _:u8) -> bool);
    }
}

#[tested_trait::test_impl]
impl Setoid for i8 {
    fn equiv(&self, b: &i8) -> bool{
        (*self).rem_euclid(5) == (*b).rem_euclid(5)
    }
}

#[tested_trait::test_impl]
impl ArbitrarySetoid for i8 {
    fn generate_equiv(&self, g: &mut quickcheck::Gen) -> i8 {
        let n: i8 = quickcheck::Arbitrary::arbitrary(g);
        (n/6).wrapping_mul(5).wrapping_add(*self % 5)
    }
}
