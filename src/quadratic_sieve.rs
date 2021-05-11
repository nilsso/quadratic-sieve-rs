#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use itertools::Itertools;

use crate::integers::Integer;
use crate::matrix::Matrix;

#[inline]
fn is_prime(x: i32) -> bool {
    (2..=(x as f32).sqrt().floor() as i32).all(|d| x % d != 0)
}

fn primes_from(x: i32) -> impl Iterator<Item = i32> {
    (x..).filter(|&x| is_prime(x))
}

fn primes() -> impl Iterator<Item = i32> {
    primes_from(2)
}

fn legendre(n: i32, p: i32) -> i32 {
    assert!(is_prime(p));
    let e = ((p - 1) / 2) as u32;
    n.pow_mod(e, p)
}

#[inline]
fn is_quadratic_residue(n: i32, p: i32) -> bool {
    if n > 2 {
        legendre(n, p) == 1
    } else {
        true
    }
}

fn legendre_primes(n: i32) -> impl Iterator<Item = i32> {
    primes().filter(move |&p| is_quadratic_residue(n, p))
}

fn factor_base(n: i32, b: usize) -> Vec<i32> {
    legendre_primes(n).take(b).collect()
}

struct Squaring {
    a: i32,
    m: i32,
}

impl Squaring {
    pub fn new(a: i32, m: i32) -> Self {
        Self { a, m }
    }
}

impl Iterator for Squaring {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.a = (self.a * self.a) % self.m;
        Some(self.a)
    }
}

fn div_while(mut x: i32, d: i32) -> (i32, i32) {
    let mut i = 0;
    while x % d == 0 {
        x /= d;
        i += 1;
    }
    (x, i)
}

/// Get two quadratic roots (or double root) of an integer modulo a prime, if they exists.
///
/// Algorithm is the Tonelli-Shanks algorithm:
/// https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm
/// http://rosettacode.org/wiki/Tonelli-Shanks_algorithm
///
/// * `n` - Integer whose roots to return
/// * `p` - Prime modulus
///
/// Returns `Option<(r1,r2)>`, two quadratic root of n modulo p (or a double root, twice), or None.
fn sqrt_mod(n: i32, p: i32) -> Option<[i32; 2]> {
    if p == 2 {
        // Any integer has a double root 0 or 1 modulo 2
        let r = n % 2;
        return Some([r, r]);
    }
    if !is_quadratic_residue(n, p) {
        return None;
    }
    // express p - 1 as q*2^s (where q is odd)
    let (q, s) = div_while(p - 1, 2);
    if s == 1 {
        let e = ((p + 1) / 4) as u32;
        let r = n.pow_mod(e, p);
        return Some([r, p - r]);
    }
    //return None;
    // find a quadratic-non residue modulo p (in this case the first)
    let z = (2..)
        .filter(|&z| !is_quadratic_residue(z, p))
        .next()
        .unwrap();
    let mut m = s;
    let mut c = z.pow_mod(q as u32, p); // pow(z, q, p)
    let mut t = n.pow_mod(q as u32, p); // pow(n, q, p)
    let mut r = n.pow_mod(((q + 1) / 2) as u32, p); // pow(n, (q+1)//2, p)
    while (t - 1) % p != 0 {
        let i = Squaring::new(t, p)
            .enumerate()
            .filter_map(|(i, t2)| ((t2 - 1) % p == 0).then_some((i + 1) as i32))
            .next()
            .unwrap();
        let b = c.pow_mod((1 << (m - i - 1)) as u32, p);
        r = (r * b) % p;
        c = (b * b) % p;
        t = (t * c) % p;
        m = i;
    }
    Some([r, p - r])
}

fn smooth(n: i32, b: usize, i: usize) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let m = (n as f32).sqrt().ceil() as i32;
    let mut sieve: Vec<i32> = (0..i).map(|x| (x as i32 + m).squared() - n).collect();
    // Collect primes p for which n has roots mod p, as well as collec the roots
    let (fb, roots): (Vec<i32>, Vec<[i32; 2]>) = primes()
        .filter_map(|p| sqrt_mod(n, p).map(|r| (p, r)))
        .take(b)
        .unzip();
    for (p, p_roots) in fb.iter().zip(roots.iter()) {
        for r in p_roots {
            let start = (r - m).rem_euclid(*p) as usize;
            for i in (start..sieve.len()).step_by(*p as usize) {
                //println!("{} {} {} {}", p, r, i, sieve[i]);
                while sieve[i] % p == 0 {
                    sieve[i] /= p;
                    //println!("reduce {} {} {} {}", p, r, i, sieve[i]);
                }
            }
        }
    }
    let xs: Vec<i32> = sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| (v.abs() == 1).then_some(i as i32 + m))
        .collect();
    let ys: Vec<i32> = xs.iter().map(|x| x.squared() - n).collect();
    (xs, ys, fb)
}

fn factor_over(n: &i32, fb: &Vec<i32>) -> (i32, Vec<(i32, u32)>) {
    let mut n = n.clone();
    let mut factorization = Vec::new();
    for p in fb.iter() {
        let mut i = 0;
        while n % p == 0 {
            n /= p;
            i += 1;
        }
        factorization.push((*p, i));
    }
    (n, factorization)
}

fn construct_exponent_matrices<const B: usize>(_ys: &Vec<i32>, _fb: &Vec<i32>)
where
    [(); B + 1]: Sized,
{
    let a: Matrix<i32, { B + 1 }, B>;
    // NOTE: OOOOOPS. m and n are non-constant
    // Matrix::<i32, m, n>::zero();
}

/// Quadratic sieve
///
/// * `n` - Integer to factor
/// * `b` - Desired factor base length
/// * `i` - Sieving interval
pub fn qs(n: i32, b: usize, i: usize) -> Result<(i32, i32), String> {
    let (xs, ys, fb) = smooth(n, b, i);
    if xs.len() <= fb.len() {
        return Err("err".to_string());
    }

    //construct_exponent_matrices(&ys, &fb);

    Ok((0, 0))
}
