use std::ops::{Div, Mul, Sub};

/// Division remainder operations.
pub trait Rem {
    fn rem(self, other: Self) -> Self;
    fn rem_euclid(self, other: Self) -> Self;
}

macro_rules! impl_rem {
    ($T:ty) => {
        impl Rem for $T {
            fn rem(self, other: Self) -> Self {
                self % other
            }
            fn rem_euclid(self, other: Self) -> Self {
                <$T>::rem_euclid(self, other)
            }
        }
    };
    ($($T:ty),*) => {
        $( impl_rem!($T);)*
    };
}

// Implement Rem for primitive types
impl_rem!(i32, u32, u64, f32, f64);

/// If values are equivalent modulo a base.
pub trait Equivalent {
    fn equivalent(&self, other: Self, base: Self) -> bool;
}

// Implement for Rem types (i.e. primitives).
impl<T: Rem + Eq + Copy> Equivalent for T {
    fn equivalent(&self, other: T, base: T) -> bool {
        self.rem_euclid(base) == other
    }
}

pub trait GCD {
    fn gcd(&self, other: Self) -> Self;
}

impl<T> GCD for T
where
    T: One + Zero + PartialOrd + Clone,
    for<'a> &'a T: Sub<T, Output = T>,
    for<'a, 'b> &'a T: Mul<&'b T, Output = T> + Div<&'b T, Output = T>,
{
    // Extended Euclidean algorithm
    fn gcd(&self, other: T) -> T {
        // [r, s, t]
        let mut prev = [self.clone(), T::ONE, T::ZERO];
        let mut curr = [other, T::ZERO, T::ONE];
        while curr[0] > T::ZERO {
            let q = &prev[0] / &curr[0];
            for i in 0..2 {
                let temp = curr[i];
                curr[i] = &prev[i] - &q * &temp;
                prev[i] = temp;
            }
        }
        prev[0]
    }
}

/// Multiplicative identity.
pub trait One {
    const ONE: Self;
}

/// Additive identity.
pub trait Zero {
    const ZERO: Self;
}

macro_rules! impl_identities {
    ($one:expr, $zero:expr, $($T:ty),*) => {
        $(
            impl One for $T {
                const ONE: $T = $one;
            }
            impl Zero for $T {
                const ZERO: $T = $zero;
            }
        )*
    };
}

impl_identities!(1, 0, i32, i64, u32, u64);
impl_identities!(1.0, 0.0, f32, f64);

pub trait Pow {
    fn pow(&self, n: Self) -> Self
    where
        Self: One;

    fn squared(&self) -> Self
    where
        Self: One + Clone + Sized,
        for<'a> &'a Self: Mul<Output = Self>,
    {
        self * self
    }
}

macro_rules! impl_pow {
    ($($T:ty),*) => {
        $(
            impl Pow for $T {
                fn pow(&self, n: Self) -> Self {
                    <$T>::pow(self.clone(), n as u32)
                }
            }
        )*
    };
}

impl_pow!(i32, i64, u32, u64);
