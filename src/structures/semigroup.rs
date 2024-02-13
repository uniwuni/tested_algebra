#[tested_trait::tested_trait]
trait Semigroup {
    fn op(self, x: Self) -> Self;

    fn prop_semigroup_op_associative(x: Self, y:Self, z:Self) -> bool where Self: Eq + Copy {
        x.op(y).op(z) == x.op(y.op(z))
    }
    #[test]
    fn semigroup_op_associative() where Self: quickcheck::Arbitrary + Eq + Copy + std::fmt::Debug {
        quickcheck::quickcheck(Self::prop_semigroup_op_associative as fn(_: Self, _:Self, _:Self) -> bool);
    }
}

#[tested_trait::test_impl]
impl Semigroup for std::num::Wrapping<i32> {
    fn op(self, x: Self) -> Self { self + x }
}
