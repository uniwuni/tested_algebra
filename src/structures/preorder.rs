use super::setoid::*;
#[tested_trait::tested_trait]
pub trait Preorder where Self: Clone + Setoid {
    fn le(&self, b: &Self) -> bool;
    fn prop_le_reflexive(a: Self) -> bool {
        *(&a.le(&a))
    }
    fn prop_le_transitive(a : Self, b: Self, c: Self) -> bool {
        if *(&a.le(&b)) && *(&b.le(&c)) {
            *(&a.le(&c))
        } else {
            true
        }
    }
    #[test]
    fn le_reflexive() where Self: quickcheck::Arbitrary + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_le_reflexive as fn(_: Self) -> bool);
    }
    #[test]
    fn le_transitive() where Self: quickcheck::Arbitrary + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_le_transitive as fn(_: Self, _: Self, _:Self) -> bool);
    }

}
#[tested_trait::tested_trait]
trait ArbitrarySetoidPreorder where Self: Preorder + ArbitrarySetoid {
    fn prop_equiv_generate_le_compat(a : Self, b : Self, size: u8) -> Result<(), (Self, Self)> {
        let mut g = quickcheck::Gen::new(size as usize);
        let a2 = Self::generate_equiv(&a, &mut g);
        let b2 = Self::generate_equiv(&b, &mut g);
        if *(&a.le(&b)) == *(&a2.le(&b2)) { Ok(()) } else {Err((a2,b2))}
    }
    #[test]
    fn equiv_generate_le_compat() where Self: std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_equiv_generate_le_compat as fn(_: Self, _: Self, _:u8) -> Result<(), (Self,Self)>);
    }
}

trait ArbitraryPreorder where Self: ArbitrarySetoidPreorder {
    /// `x.generate_below(g)` should yield a random element of {y | y <= x}.
    fn generate_below(&self, g: &mut quickcheck::Gen) -> Self;
    /// `x.generate_below(g)` should yield a random element of {y | x <= y}.
    fn generate_above(&self, g: &mut quickcheck::Gen) -> Self;
    fn prop_generate_below_le(a : Self, size: u8) -> Result<(), Self> where Self: quickcheck::Arbitrary {
        let mut g = quickcheck::Gen::new(size as usize);
        let b = Self::generate_below(&a, &mut g);
        if *(&b.le(&a)) { Ok(()) } else {Err(b)}
    }
    fn prop_generate_above_le(a : Self, size: u8) -> Result<(), Self> where Self: quickcheck::Arbitrary {
        let mut g = quickcheck::Gen::new(size as usize);
        let b = Self::generate_above(&a, &mut g);
        if *(&a.le(&b)) { Ok(()) } else {Err(b)}
    }
}
