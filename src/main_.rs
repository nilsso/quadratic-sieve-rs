#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(bool_to_option)]
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
#![feature(trait_alias)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
//#![allow(late_bound_lifetime_arguments)]

use std::cmp::PartialEq;
use std::default::Default;
use std::iter::repeat;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::{fmt, fmt::Display};

use itertools::{iproduct, join};

pub type Shape = (usize, usize);

pub trait Zero {
    type Element;

    const ZERO: Self::Element;
}

pub trait One {
    type Element;

    const ONE: Self::Element;
}

impl Zero for u32 {
    type Element = u32;

    const ZERO: u32 = 0;
}

impl One for u32 {
    type Element = u32;

    const ONE: u32 = 1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CongruenceClass<const M: u32>(u32);

impl<const M: u32> CongruenceClass<M> {
    pub fn iter() -> impl Iterator<Item = u32> {
        (M..).step_by(M as usize)
    }

    pub fn ith(i: usize) -> u32 {
        (i as u32) * M
    }

    pub fn first_prime() -> u32 {
        todo!()
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

impl<const M: u32> Add for CongruenceClass<M> {
    type Output = CongruenceClass<M>;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % M)
    }
}

impl<const M: u32> Sub for CongruenceClass<M> {
    type Output = CongruenceClass<M>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self((self.0 - rhs.0) % M)
    }
}

impl<const M: u32> Display for CongruenceClass<M> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}_{}", self.0, M)
    }
}

#[derive(Copy, Clone)]
pub struct QuotientRing<const M: u32>();

impl<const M: u32> Zero for QuotientRing<M> {
    type Element = CongruenceClass<M>;

    const ZERO: CongruenceClass<M> = CongruenceClass::<M>::ZERO;
}

impl<const M: u32> One for QuotientRing<M> {
    type Element = CongruenceClass<M>;

    const ONE: CongruenceClass<M> = CongruenceClass::<M>::ONE;
}

impl<const M: u32> Add<u32> for QuotientRing<M> {
    type Output = CongruenceClass<M>;

    fn add(self, rhs: u32) -> Self::Output {
        CongruenceClass(rhs % M)
    }
}

#[rustfmt::skip]
pub trait IntegerLike =
    Zero<Element = Self>
    + One<Element = Self>
    + Sub<Output = Self>
    + Copy
    + Clone;

pub type Buffer<T, const R: usize, const C: usize> = [T; R * C];

#[derive(Clone)]
pub struct Matrix<T, const R: usize, const C: usize>
where
    T: Clone,
    [T; R * C]: Sized,
{
    pub elements: Buffer<T, R, C>,
}

/*
 *impl<'a, T, const R: usize, const C: usize> Iterator for RowIter<'a, T, R, C>
 *where
 *    [T; R * C]: Sized,
 *{
 *    type Item = &'a [T];
 *
 *    fn next(&mut self) -> Option<Self::Item> {
 *        let i = self.curr * C;
 *        self.curr += 1;
 *        self.mat.elements.get(i..i + C)
 *    }
 *}
 */

/*
 *pub struct ColMajorIterMut<T, const R: usize, const C: usize> {
 *    mat: &mut Matrix<T, R, C>,
 *}
 */

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Clone,
    [T; R * C]: Sized,
{
    pub fn from_array(elements: [T; R * C]) -> Self {
        Self { elements }
    }

    pub fn index(&self, i: usize, j: usize) -> usize {
        j + i * C
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        let index = self.index(i, j);
        self.elements.get(index)
    }

    pub fn get_unchecked(&self, i: usize, j: usize) -> &T {
        let index = self.index(i, j);
        &self.elements[index]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        let index = self.index(i, j);
        self.elements.get_mut(index)
    }

    pub fn get_mut_unchecked(&mut self, i: usize, j: usize) -> &mut T {
        let index = self.index(i, j);
        &mut self.elements[index]
    }

    /// Mutable iterator over elements in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    /// Mutable iterator over elements in row-major order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elements.iter_mut()
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    pub fn iter_col_major(&self) -> impl Iterator<Item = &T> {
        (0..C).flat_map(move |j| (0..R).map(move |i| self.get_unchecked(i, j)))
    }

    /// Iterator over elements in the main diagonal.
    pub fn iter_diag(&self) -> impl Iterator<Item = &T> {
        self.iter().step_by(C + 1)
    }

    /// Mutable iterator over elements in the main diagonal.
    pub fn iter_diag_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut().step_by(C + 1)
    }

    pub fn col(&self, j: usize) -> Result<[&T; R], ()> {
        (j < C).then_some(self.col_unchecked(j)).ok_or(())
    }

    pub fn col_unchecked(&self, j: usize) -> [&T; R] {
        use std::mem::MaybeUninit;

        let mut col: [MaybeUninit<&T>; R] = MaybeUninit::uninit_array();
        for i in 0..R {
            col[i] = MaybeUninit::new(&self.elements[i * C + j]);
        }
        unsafe { MaybeUninit::array_assume_init(col) }
    }

    pub fn col_mut(&mut self, j: usize) -> Result<[&mut T; R], ()> {
        (j < C).then_some(self.col_mut_unchecked(j)).ok_or(())
    }

    pub fn col_mut_unchecked(&mut self, j: usize) -> [&mut T; R] {
        use std::mem::MaybeUninit;

        let ptr = self.elements.as_mut_ptr();
        let mut col: [MaybeUninit<&mut T>; R] = MaybeUninit::uninit_array();
        unsafe {
            for i in 0..R {
                col[i] = MaybeUninit::new(ptr.add(i * C + j).as_mut().unwrap());
            }
            MaybeUninit::array_assume_init(col)
        }
    }

    /*
     *pub fn iter_cols(&self) -> impl Iterator<Item = [&T; R]> {
     *    (0..C).map(move |j| self.col_unchecked(j))
     *}
     */
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: IntegerLike + Clone,
    [T; R * C]: Sized,
{
    pub fn zeros() -> Self {
        let elements = [T::ZERO; R * C];
        Self { elements }
    }

    pub fn eye() -> Self {
        let mut res = Self::zeros();
        for a in res.iter_diag_mut() {
            *a = T::ONE;
        }
        res
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: IntegerLike + Clone + Default,
    [T; R * C]: Sized,
{
    pub fn new() -> Self {
        Self {
            elements: [T::default(); R * C],
        }
    }
}

impl<T, const R: usize, const C: usize> Display for Matrix<T, R, C>
where
    T: IntegerLike + Clone + Display,
    [T; R * C]: Sized,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let strings: Vec<String> = self.iter().map(T::to_string).collect();
        let max_width = strings.iter().map(String::len).max().unwrap();

        macro_rules! write_end {
            ($l:literal, $r:literal) => {
                write!(fmt, $l)?;
                for _ in 0..C {
                    write!(fmt, "{:>w$} ", "", w = max_width)?;
                }
                writeln!(fmt, $r)?;
            };
        }

        write_end!("┌ ", "┐");
        for i in 0..R {
            write!(fmt, "│ ")?;
            for j in 0..C {
                write!(fmt, "{:>w$} ", strings[j + i * R], w = max_width)?;
            }
            writeln!(fmt, "│")?;
        }
        write_end!("└ ", "┘");
        Ok(())
    }
}

pub trait Conjugate {
    type Output;

    fn conj(&self) -> Self::Output;
}

impl<T, const R: usize, const C: usize, const N: usize> Conjugate for Matrix<T, R, C>
where
    T: Clone + Conjugate,
    [T; R * C]: Sized,
    [T; C * R]: Sized,
{
    type Output = Matrix<T, C, R>;

    fn conj(&self) -> Self::Output {
        use std::mem::swap;

        let mut elements = self.elements.clone();
        let n = R * C - 1;
        for i in 0..R * C {
            let j = (i * R) % n;
            swap(&mut elements[i], &mut elements[j]);
        }
        Matrix::<T, C, R>::from_array(elements)
    }
}

pub trait EchelonForm {
    type Item;

    fn to_echelon(&self) -> Self;
    fn to_echelon_mod(&self, modulus: Self::Item) -> Self;
}

fn main() {
    #[rustfmt::skip]
    let emat = Matrix::<u32, 3, 3>::from_array([
        0, 1, 2,
        3, 4, 5,
        6, 7, 7,
    ]);
    //for a in emat.
    //for &mut a in emat.col_mut(0).iter_mut() {
    //*a = 0_u32;
    //}
    //println!("{}", emat);
    //for j in 0..5 {
    //println!("{:?}", emat.col(j));
    //}
    //println!("{}", emat);
    //for a in emat.iter_col_major() {
    //println!("{}", a);
    //}
}
