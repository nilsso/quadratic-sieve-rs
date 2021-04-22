#![feature(bool_to_option)]
//pub mod primes;
//pub mod rings;
//pub mod traits;

//use rings::RingElement;
//use traits::Equivalent;

//pub mod prelude {
//pub use crate::{
//primes::{PrimeBank, PrimeIter},
//rings::{Ring, RingElement},
//traits::{Equivalent, One, Pow, Rem, Zero, GCD},
//QuadraticResidue,
//};
//}

//pub trait QuadraticResidue {
//fn is_quadratic_residue(self) -> bool;
//}

//impl QuadraticResidue for (i32, i32) {
//fn is_quadratic_residue(self) -> bool {
//let (n, base) = self;

//(0..base).any(|x| {
//println!("{}", x * x);
//(x * x).equivalent(n.rem_euclid(base), base)
//})
//}
//}

//impl QuadraticResidue for RingElement {
//fn is_quadratic_residue(self) -> bool {
//let RingElement { value, base, .. } = self;
//(value, base).is_quadratic_residue()
//}
//}
