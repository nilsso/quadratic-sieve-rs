use std::ops::Add;

use crate::congruence_class::CongruenceClass;
use crate::identity::{One, Zero};

pub struct QuotientGroup<const M: u32>;

impl<const M: u32> Zero for QuotientGroup<M> {
    type Element = CongruenceClass<M>;

    const ZERO: CongruenceClass<M> = CongruenceClass::new(0);
}

impl<const M: u32> One for QuotientGroup<M> {
    type Element = CongruenceClass<M>;

    const ONE: CongruenceClass<M> = CongruenceClass::new(1);
}

impl<const M: u32> Add<u32> for QuotientGroup<M> {
    type Output = CongruenceClass<M>;

    fn add(self, rhs: u32) -> CongruenceClass<M> {
        CongruenceClass::new(rhs % M)
    }
}
