#![feature(bool_to_option)]
#![feature(step_trait)]
#![allow(unused_imports)]
#![allow(dead_code)]
//use itertools::iproduct;
//use quadratic_sieve::prelude::*;

//use std::cmp::Eq;
use std::iter::Step;
use std::ops::{Mul, Rem};

trait One {
    const ONE: Self;
}

impl One for i32 {
    const ONE: i32 = 1;
}

trait Residue {
    fn is_quadratic_residue(&self, m: &Self) -> bool;

    fn quadratic_residues(m: &Self) -> Vec<Self>
    where
        Self: One + Step + Sized,
    {
        (Self::ONE..m.clone())
            .filter(|q| q.is_quadratic_residue(m))
            .collect()
    }

    fn quadratic_nonresidues(m: &Self) -> Vec<Self>
    where
        Self: One + Step + Sized,
    {
        (Self::ONE..m.clone())
            .filter(|q| !q.is_quadratic_residue(m))
            .collect()
    }
}

impl<T> Residue for T
where
    T: One + Mul<Output = T> + Rem + Eq + Step + Clone,
    for<'b> T: Rem<&'b T, Output = T>,
    for<'a, 'b> &'a T: Mul<&'b T, Output = T> + Rem<&'b T, Output = T> + Eq,
{
    fn is_quadratic_residue(&self, m: &T) -> bool
    where
        T: One + Mul<Output = T> + Rem + Eq + Step + Clone,
        for<'b> T: Rem<&'b T, Output = T>,
        for<'a, 'b> &'a T: Mul<&'b T, Output = T> + Rem<&'b T, Output = T> + Eq,
    {
        for x in T::ONE..m.clone() {
            if &((x.clone() * x) % m) == self {
                return true;
            }
        }
        false
    }
}

fn main() {
    let m = 10;

    println!("{:?}", i32::quadratic_residues(&m));
    println!("{:?}", i32::quadratic_nonresidues(&m));

    //for q in 1..=m {
    //println!("{} {}", &q, is_quadratic_residue(&q, &m));
    //}
    //use std::str::FromStr;

    //use quadratic_sieve::GCD;

    //let mut args = std::env::args().skip(1);
    //let a = i32::from_str(&args.next().unwrap()).unwrap();
    //let b = i32::from_str(&args.next().unwrap()).unwrap();

    //println!("{:?}", a.gcd(b));

    //let n = 2041;
    //let n_root = (n as f32).sqrt().floor() as i32;
    //let zs: Vec<i32> = (-2..2).map(|a| n_root + a).collect();

    //let base_factorize = |mut a: i32| -> Vec<(i32, u16)> {
    //a *= a;
    //while a > n {
    //a -= n;
    //}
    //factorize(a)
    //};

    //for z in zs {
    //let z2 = z * z - n;
    //println!("{} {} {:?}", z, z2, factorize(z2));
    //}

    //let a = base_factorize(z4 * z4);

    //println!("{:?}", factorize(a));

    //let a = 3125;
    //let b = 9987;

    //let ainv = gcd(a, b).unwrap();

    //println!("{} {}", ainv, (a * ainv) % b);
}
