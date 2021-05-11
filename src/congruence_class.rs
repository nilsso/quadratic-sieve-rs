#[allow(unused_imports)]
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::{fmt, fmt::Display};

use crate::identity::{One, Zero};
use crate::integers::GCD;

/// Congruence class
///
/// # Examples
///
/// We can add congruence classes if they have the same modulus:
/// ```
/// use quadratic_sieve::congruence_class::CongruenceClass;
///
/// let a = CongruenceClass::<5>::new(1);
/// let b = CongruenceClass::<5>::new(4);
/// assert_eq!(a + b, CongruenceClass::<5>::new(0));
/// ```
/// But not if they have different modulus:
/// ```skip
/// use quadratic_sieve::congruence_class::CongruenceClass;
///
/// let a = CongruenceClass::<5>::new(1);
/// let b = CongruenceClass::<6>::new(5);
/// a + b; // fails to compile, since 5 != 6
/// ```
///
/// NOTE:
/// Unfortunately const generics cannot depend on generic types yet,
/// so the following, which we'd want, is unpermitted:
/// ```skip
/// pub struct CongruenceClass<T, const M: T>(T);
/// ```
/// This is expected to be possible in the "near" future.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct CongruenceClass<const M: u32>(u32);

impl<const M: u32> CongruenceClass<M> {
    pub const fn new(value: u32) -> Self {
        CongruenceClass(value % M)
    }
}

impl<const M: u32> Zero for CongruenceClass<M> {
    type Element = CongruenceClass<M>;

    const ZERO: CongruenceClass<M> = CongruenceClass(0);
}

impl<const M: u32> One for CongruenceClass<M> {
    type Element = CongruenceClass<M>;

    const ONE: CongruenceClass<M> = CongruenceClass(1);
}

macro_rules! impl_op {
    (@variant $TL:ty, $TR:ty, $Op:ident, $op:tt) => {
        impl<'a, 'b, const M: u32> $Op<$TR> for $TL {
            type Output = CongruenceClass<M>;

            fn $op(self, rhs: $TR) -> Self::Output {
                CongruenceClass::<M>($Op::$op(self.0, rhs.0) % M)
            }
        }
    };
    ($Op:ident, $op:tt) => {
        impl_op!(@variant     CongruenceClass<M>,     CongruenceClass<M>, $Op, $op);
        impl_op!(@variant &'a CongruenceClass<M>,     CongruenceClass<M>, $Op, $op);
        impl_op!(@variant     CongruenceClass<M>, &'b CongruenceClass<M>, $Op, $op);
        impl_op!(@variant &'a CongruenceClass<M>, &'b CongruenceClass<M>, $Op, $op);
    };
    ($(($Op:ident, $op:tt)),*) => {
        $(
            impl_op!($Op, $op);
        )*
    };
}

impl_op!((Add, add), (Mul, mul), (Div, div), (Rem, rem));

macro_rules! impl_sub {
    (@variant $TL:ty, $TR:ty) => {
        impl<'a, 'b, const M: u32> Sub<$TR> for $TL {
            type Output = CongruenceClass<M>;

            fn sub(self, rhs: $TR) -> Self::Output {
                // TODO: Do this with adding multiples of self.0, then subtracting
                CongruenceClass::<M>(((self.0 as i32 - rhs.0 as i32).rem_euclid(M as i32)) as u32)
            }
        }
    };
    () => {
        impl_sub!(@variant     CongruenceClass<M>,     CongruenceClass<M>);
        impl_sub!(@variant &'a CongruenceClass<M>,     CongruenceClass<M>);
        impl_sub!(@variant     CongruenceClass<M>, &'b CongruenceClass<M>);
        impl_sub!(@variant &'a CongruenceClass<M>, &'b CongruenceClass<M>);
    };
}

impl_sub!();

use std::iter::Sum;

impl<const M: u32> Sum for CongruenceClass<M> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = CongruenceClass<M>>,
    {
        CongruenceClass(iter.fold(0, |a, b| -> u32 { (a + b.0) % M }))
    }
}

impl<const M: u32> GCD for CongruenceClass<M> {
    fn gcd(self, rhs: Self) -> Self {
        Self(self.0.gcd(rhs.0))
    }
}

impl<const M: u32> Display for CongruenceClass<M> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}_{}", self.0, M)
    }
}

impl<const M: u32> From<u32> for CongruenceClass<M> {
    fn from(a: u32) -> Self {
        CongruenceClass::new(a)
    }
}

#[macro_export]
macro_rules! cc {
    ($a:literal, $M:literal) => {
        CongruenceClass::<$M>::new($a)
    };
}

#[macro_export]
macro_rules! cc_array {
    ($m:literal,[$($x:literal),*$(,)?]) => {
        [$(cc!($x,$m),)*]
    };
    ($m:literal,[$([$($x:literal),*$(,)?]),*$(,)?]) => {
        [$(cc_array!($m,[$($x),*]),)*]
    };
}
