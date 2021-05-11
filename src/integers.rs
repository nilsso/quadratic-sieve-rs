use std::mem::swap;
use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::identity::{One, Zero};

pub trait ExtEuclid
where
    Self: Sized,
{
    fn ext_euclid(self, rhs: Self) -> [Self; 3];
}

impl<T> ExtEuclid for T
where
    T: Zero<Element = T> + One<Element = T> + PartialEq + Clone,
    for<'a> &'a T: Add<T, Output = T> + Sub<T, Output = T>,
    for<'b> T: Rem<&'b T, Output = T>,
    for<'a, 'b> &'a T: Mul<&'b T, Output = T> + Div<&'b T, Output = T>,
{
    fn ext_euclid(self, rhs: Self) -> [Self; 3] {
        let mut prev = [self, Self::ONE, Self::ZERO];
        let mut curr = [rhs.clone(), Self::ZERO, Self::ONE];
        while curr[0] != Self::ZERO {
            let q = &prev[0] / &curr[0];
            for i in 0..=2 {
                swap(&mut prev[i], &mut curr[i]);
                curr[i] = &curr[i] - &q * &prev[i];
            }
        }
        prev
    }
}

pub trait GCDEuclid: ExtEuclid {
    fn gcd_euclid(self, rhs: Self) -> Self;
}

impl<T> GCDEuclid for T
where
    T: ExtEuclid,
{
    /// # Examples
    ///
    /// Will work for an signed integer
    /// ```
    /// use quadratic_sieve::integers::GCDEuclid;
    ///
    /// let a: i32 = 23 * 8; // 184
    /// let b: i32 = 23 * 9; // 207
    /// assert_eq!(a.gcd_euclid(b), 23);
    /// assert_eq!(b.gcd_euclid(a), 23);
    /// ```
    /// But is liable to fail for unsigned integers as a result
    /// of subtractions in the quotient sequence.
    /// ```should_panic
    /// use quadratic_sieve::integers::GCDEuclid;
    ///
    /// let a: u32 = 4; // 184
    /// let b: u32 = 7; // 207
    /// a.gcd_euclid(b);
    /// ```
    fn gcd_euclid(self, rhs: T) -> T {
        let [d, _, _] = self.ext_euclid(rhs);
        d
    }
}

pub trait GCD {
    fn gcd(self, rhs: Self) -> Self;
}

// Borrowed from the wonderful
// https://docs.rs/num-integer/0.1.44/src/num_integer/lib.rs.html#462
macro_rules! impl_gcd_unsigned {
    ($T:ty) => {
        impl GCD for $T {
            fn gcd(self, rhs: Self) -> Self {
                let mut m = self;
                let mut n = rhs;
                if m == 0 || n == 0 {
                    return m | n;
                }
                let shift = (m | n).trailing_zeros();
                m >>= m.trailing_zeros();
                n >>= n.trailing_zeros();
                while m != n {
                    if m > n {
                        m -= n;
                        m >>= m.trailing_zeros();
                    } else {
                        n -= m;
                        n >>= n.trailing_zeros();
                    }
                }
                m << shift
            }
        }
    };
    ($($T:ty),*) => { $( impl_gcd_unsigned!($T); )* };
}
macro_rules! impl_gcd_signed {
    ($T:ty) => {
        impl GCD for $T {
            fn gcd(self, rhs: Self) -> Self {
                let mut m = self;
                let mut n = rhs;
                if m == 0 || n == 0 {
                    return (m | n).abs();
                }
                let shift = (m | n).trailing_zeros();
                if m == Self::min_value() || n == Self::min_value() {
                    return ((1 as $T) << shift).abs();
                }
                m = m.abs();
                n = n.abs();
                m >>= m.trailing_zeros();
                n >>= n.trailing_zeros();
                while m != n {
                    if m > n {
                        m -= n;
                        m >>= m.trailing_zeros();
                    } else {
                        n -= m;
                        n >>= n.trailing_zeros();
                    }
                }
                m << shift
            }
        }
    };
    ($($T:ty),*) => { $( impl_gcd_signed!($T); )* };
}

impl_gcd_unsigned!(u8, u16, u32, u64);
impl_gcd_signed!(i8, i16, i32, i64);

pub trait LCM: GCD {
    fn lcm(self, rhs: Self) -> Self;
}

use std::fmt::Display;

impl<T> LCM for T
where
    T: Mul<Output = T> + Div<Output = T> + GCD + Clone + Display,
{
    fn lcm(self, rhs: Self) -> Self {
        let gcd = self.clone().gcd(rhs.clone());
        (self * rhs) / gcd
    }
}

pub trait Inverse: ExtEuclid
where
    Self: Sized,
{
    fn inv(self, modulo: Self) -> Option<Self>;
}

impl<T> Inverse for T
where
    T: ExtEuclid + One<Element = T> + Rem<Output = T> + PartialEq + Clone,
{
    fn inv(self, modulo: Self) -> Option<Self> {
        let [d, x, _] = self.ext_euclid(modulo.clone());
        (d == T::ONE).then_some(x % modulo)
    }
}

pub trait Integer: Zero<Element = Self> + One<Element = Self>
where
    Self: Sized,
{
    fn abs(&self) -> Self;
    //fn rem_euclid(&self, m: &Self) -> Self;
    fn pow(&self, e: u32) -> Self;
    fn log2(&self) -> Self;
    //fn bit;
    fn bit_length(&self) -> Self;
    fn pow_mod(&self, e: u32, m: Self) -> Self;
    //fn sqrt_mod(&self, m: Self) -> Self;

    fn squared(&self) -> Self {
        self.pow(2)
    }
}

macro_rules! impl_integer {
    ($T:ty) => {
        fn pow(&self, e: u32) -> Self {
            <$T>::pow(self.clone(), e)
        }
        //fn rem_euclid(&self, m: &Self) -> Self {
        //<$T>::rem_euclid(self.clone(), m)
        //}
        fn log2(&self) -> Self {
            (*self as f32).log2() as Self
        }
        fn bit_length(&self) -> Self {
            self.abs().log2() + 1
        }
        fn pow_mod(&self, mut e: u32, m: Self) -> Self {
            let mut b = self % &m;
            let mut res = 1;
            while e > 0 {
                if &e % 2 == 1 {
                    res = (res * &b) % &m;
                }
                e = e >> 1;
                b = (&b * &b) % &m;
            }
            res
        }
    };
}

macro_rules! impl_integer_signed {
    ($T:ty) => {
        impl Integer for $T {
            fn abs(&self) -> Self {
                <$T>::abs(self.clone())
            }
            impl_integer!($T);
        }
    };
    ($($T:ty),*) => { $( impl_integer_signed!($T); )* };
}

macro_rules! impl_integer_unsigned {
    ($T:ty) => {
        impl Integer for $T
        {
            fn abs(&self) -> Self {
                self.clone()
            }
            impl_integer!($T);
        }
    };
    ($($T:ty),*) => { $( impl_integer_unsigned!($T); )* };
}

impl_integer_signed!(i8, i16, i32, i64);
impl_integer_unsigned!(u8, u16, u32, u64);
