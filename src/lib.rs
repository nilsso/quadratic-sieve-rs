#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(bool_to_option)]
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
#![feature(trait_alias)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![recursion_limit = "10"]
#![allow(incomplete_features)]

pub mod complex;
pub mod congruence_class;
pub mod conjugate;
pub mod etc;
pub mod identity;
pub mod integers;
pub mod matrix;
pub mod quadratic_sieve;
pub mod quotient_group;

pub mod prelude {
    #[rustfmt::skip]
    pub use crate::{
        {cc, cc_array}, // CongruenceClass macro
        congruence_class::CongruenceClass,
        conjugate::Conjugate,
        identity::{One, Zero},
        integers::{Inverse, GCD, LCM, Integer},
        matrix::Matrix,
        quotient_group::QuotientGroup,
        quadratic_sieve::qs,
    };
}
