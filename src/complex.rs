use std::ops::Neg;
use std::{fmt, fmt::Display};

use crate::conjugate::Conjugate;

#[derive(Copy, Clone)]
pub struct Complex<T>(T, T);

impl<T> Complex<T> {
    pub const fn new(r: T, i: T) -> Self {
        Self(r, i)
    }
}

impl<T> Conjugate for Complex<T>
where
    T: Neg<Output = T> + Clone,
{
    type Output = Self;

    fn conj(&self) -> Complex<T> {
        Complex(self.0.clone(), -self.1.clone())
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({})+({})i", self.0, self.1)
    }
}
