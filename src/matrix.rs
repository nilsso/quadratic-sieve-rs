use std::mem::MaybeUninit;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::{fmt, fmt::Display};

use array_init::array_init;
use itertools::{iproduct, izip};

use crate::conjugate::Conjugate;
use crate::identity::{One, Zero};
use crate::integers::LCM;

struct MatrixBuffer<T, const M: usize, const N: usize>
where
    T: Clone,
{
    pub rows: Vec<[MaybeUninit<T>; N]>,
}

impl<T, const M: usize, const N: usize> MatrixBuffer<T, M, N>
where
    T: Clone,
{
    pub unsafe fn new() -> Self {
        let mut rows = Vec::with_capacity(M);
        for _ in 0..M {
            rows.push(MaybeUninit::uninit_array());
        }
        Self { rows }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut MaybeUninit<T>> {
        self.rows.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub unsafe fn finish(self) -> [[T; N]; M] {
        let mut buffer: [MaybeUninit<[T; N]>; M] = MaybeUninit::uninit_array();
        for (i, row) in self.rows.into_iter().enumerate() {
            let row = MaybeUninit::array_assume_init(row);
            buffer[i] = MaybeUninit::new(row);
        }
        MaybeUninit::array_assume_init(buffer)
    }
}

pub trait Primitive<T> = PrimitiveRef<T, T> + Zero<Element = T> + One<Element = T> + Clone;

pub trait PrimitiveRef<T, R> = Add<R, Output = T>
    + Sub<R, Output = T>
    + Mul<R, Output = T>
    + Div<R, Output = T>
    + Rem<R, Output = T>
    + PartialEq;

#[derive(Clone, Debug)]
pub struct Matrix<T, const M: usize, const N: usize>
where
    T: Clone,
{
    pub rows: [[T; N]; M],
}

pub type ColVector<T, const M: usize> = Matrix<T, M, 1>;
pub type RowVector<T, const N: usize> = Matrix<T, 1, N>;

// TODO:
// - Matrix slices (rows, columns, submatrices)
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Clone,
{
    pub fn from_array(rows: [[T; N]; M]) -> Self {
        Self { rows }
    }

    /// Iterator over elements in row-major order.
    pub fn iter_row_major(&self) -> impl Iterator<Item = &T> {
        self.rows.iter().flat_map(|row| row.iter())
    }

    /// Mutable iterator over elements in row-major order.
    pub fn iter_mut_row_major(&mut self) -> impl Iterator<Item = &mut T> {
        self.rows.iter_mut().flat_map(|row| row.iter_mut())
    }

    /// Iterator over elements in column-major order.
    pub fn iter_col_major(&self) -> impl Iterator<Item = &T> {
        (0..N).flat_map(move |j| self.rows.iter().map(move |row| &row[j]))
    }

    /// Mutable iterator over matrix elements in column-major order.
    pub fn iter_mut_col_major(&mut self) -> impl Iterator<Item = &mut T> {
        let rows_ptr = self.rows.as_mut_ptr();
        iproduct!(0..N, 0..M).map(move |(j, i)| {
            let row = unsafe { rows_ptr.add(i).as_mut().unwrap() };
            &mut row[j]
        })
    }

    /// Iterator over matrix elements of the i'th row.
    pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &T> {
        self.rows[i].iter()
    }

    /// Mutable iterator over matrix elements of the i'th row.
    pub fn iter_mut_row(&mut self, i: usize) -> impl Iterator<Item = &mut T> {
        self.rows[i].iter_mut()
    }

    /// Iterator over matrix elements of the j'th column.
    pub fn iter_col(&self, j: usize) -> impl Iterator<Item = &T> {
        self.rows.iter().map(move |row| &row[j])
    }

    /// Mutable iterator over matrix elements of the j'th column.
    pub fn iter_mut_col(&mut self, j: usize) -> impl Iterator<Item = &mut T> {
        self.rows.iter_mut().map(move |row| &mut row[j])
    }

    /// Iterator over matrix elements on the main diagonal.
    pub fn iter_diag(&self) -> impl Iterator<Item = &T> {
        self.rows.iter().enumerate().map(|(i, row)| &row[i])
    }

    /// Mutable iterator over matrix elements on the main diagonal.
    pub fn iter_mut_diag(&mut self) -> impl Iterator<Item = &mut T> {
        self.rows.iter_mut().enumerate().map(|(i, row)| &mut row[i])
    }

    /// Iterator over matrix elements on the i'th superdiagonal.
    pub fn iter_supdiag(&self, j: usize) -> impl Iterator<Item = &T> {
        self.rows
            .iter()
            .enumerate()
            .filter_map(move |(i, row)| row.get(i + j))
    }

    /// Mutable iterator over matrix elements on the i'th superdiagonal.
    pub fn iter_mut_supdiag(&mut self, j: usize) -> impl Iterator<Item = &mut T> {
        self.rows
            .iter_mut()
            .enumerate()
            .filter_map(move |(k, row)| row.get_mut(k + j))
    }

    /// Iterator over matrix elements on the i'th subdiagonal.
    pub fn iter_subdiag(&self, i: usize) -> impl Iterator<Item = &T> {
        self.rows
            .iter()
            .skip(i)
            .enumerate()
            .filter_map(move |(k, row)| row.get(k))
    }

    /// Mutable iterator over matrix elements on the i'th subdiagonal.
    pub fn iter_mut_subdiag(&mut self, i: usize) -> impl Iterator<Item = &mut T> {
        self.rows
            .iter_mut()
            .skip(i)
            .enumerate()
            .filter_map(move |(k, row)| row.get_mut(k))
    }

    pub fn from_value(value: T) -> Self {
        let mut rows = unsafe { MatrixBuffer::<T, M, N>::new() };
        for a in rows.iter_mut() {
            *a = MaybeUninit::new(value.clone());
        }
        let rows = unsafe { rows.finish() };
        Matrix::<T, M, N> { rows }
    }

    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut rows = unsafe { MatrixBuffer::<T, N, M>::new() };
        for (r, a) in rows.iter_mut().zip(self.iter_col_major()) {
            *r = MaybeUninit::new(a.clone());
        }
        let rows = unsafe { rows.finish() };
        Matrix::from_array(rows)
    }
}

impl<T, const M: usize, const N: usize> PartialEq for Matrix<T, M, N>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter_row_major()
            .zip(other.iter_row_major())
            .all(|(a, b)| a == b)
    }
}

/// TODO: Since I can't yet construct a constant buffer of only zeroes,
/// I can't implement Zero or One for Matrix.
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Zero<Element = T> + Clone,
{
    pub fn zeroes() -> Self {
        let rows = array_init(|_| array_init(|_| T::ZERO));
        Self::from_array(rows)
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Zero<Element = T> + PartialEq + Clone,
{
    /// Find index $i$ of first row with $i\ge s$
    /// that is non-zero in its $j$'th component.
    /// TODO: Rename to something like "at or after"
    pub fn find_nonzero_in_col_after(&self, j: usize, s: usize) -> Option<usize> {
        self.rows
            .iter()
            .enumerate()
            .skip(s)
            .find_map(|(i, row)| (row[j] != T::ZERO).then_some(i))
    }

    /// Find index $i$ of first row that is non-zero in its $j$'th component.
    pub fn find_nonzero_in_col(&self, j: usize) -> Option<usize> {
        self.find_nonzero_in_col_after(j, 0)
    }

    /// Find index $j$ of first column with $j\ge s$
    /// that is non-zero in its $i$'th component.
    /// TODO: Rename to something like "at or after"
    pub fn find_nonzero_in_row_after(&self, i: usize, s: usize) -> Option<usize> {
        self.rows[i]
            .iter()
            .enumerate()
            .skip(s)
            .find_map(|(j, x)| (x != &T::ZERO).then_some(j))
    }

    /// Find index $j$ of first column that is non-zero in its $i$'th component.
    pub fn find_nonzero_in_row(&self, i: usize) -> Option<usize> {
        self.find_nonzero_in_row_after(i, 0)
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Zero<Element = T> + One<Element = T> + Clone,
{
    pub fn eye() -> Self {
        let mut res = Self::zeroes();
        res.iter_mut_diag().for_each(|a| *a = T::ONE);
        res
    }

    pub fn augment(&self) -> Matrix<T, M, { N + M }> {
        let mut rows = unsafe { MatrixBuffer::<T, M, { N + M }>::new() };
        {
            let mut aug_iter = rows.iter_mut();
            let mut self_iter = self.iter_row_major();
            for i in 0..M {
                for _ in 0..N {
                    let r = aug_iter.next().unwrap();
                    let a = self_iter.next().unwrap();
                    *r = MaybeUninit::new(a.clone());
                }
                for _ in N..(N + i) {
                    let r = aug_iter.next().unwrap();
                    *r = MaybeUninit::new(T::ZERO);
                }
                let r = aug_iter.next().unwrap();
                *r = MaybeUninit::new(T::ONE);
                for _ in (N + i + 1)..(N + M) {
                    let r = aug_iter.next().unwrap();
                    *r = MaybeUninit::new(T::ZERO);
                }
            }
        }
        let rows = unsafe { rows.finish() };
        Matrix::from_array(rows)
    }
}

// Helper for implementing addition/subtraction on matrix reference variants
macro_rules! impl_op {
    (@variant $TL:ty, $TR:ty, $Op:ident, $op:tt) => {
        impl<'a, 'b, T, const M: usize, const N: usize> $Op<$TR> for $TL
        where
            T: Primitive<T>,
            for<'c, 'd> &'c T: PrimitiveRef<T, &'d T>,
        {
            type Output = Matrix<T, M, N>;

            fn $op(self, rhs: $TR) -> Matrix<T, M, N> {
                let mut rows = unsafe { MatrixBuffer::<T, M, N>::new() };
                let (i, j, k) = (rows.iter_mut(), self.iter_row_major(), rhs.iter_row_major());
                for (r, a, b) in izip!(i, j, k) {
                    *r = MaybeUninit::new($Op::$op(a, b));
                }
                let rows = unsafe { rows.finish() };
                Matrix::from_array(rows)
            }
        }
    };
    ($Op:ident, $op:tt) => {
        impl_op!(@variant     Matrix<T, M, N>,     Matrix<T, M, N>, $Op, $op);
        impl_op!(@variant &'a Matrix<T, M, N>,     Matrix<T, M, N>, $Op, $op);
        impl_op!(@variant     Matrix<T, M, N>, &'b Matrix<T, M, N>, $Op, $op);
        impl_op!(@variant &'a Matrix<T, M, N>, &'b Matrix<T, M, N>, $Op, $op);
    };
}

macro_rules! impl_mul {
    (@variant $TL:ty, $TR:ty) => {
        impl<'a, 'b, T, const M: usize, const N: usize, const P: usize> Mul<$TR> for $TL
        where
            T: Primitive<T> + std::iter::Sum,
            for<'c, 'd> &'c T: PrimitiveRef<T, &'d T>,
        {
            type Output = Matrix<T, M, P>;

            fn mul(self, rhs: $TR) -> Matrix<T, M, P> {
                let mut rows = unsafe { MatrixBuffer::<T, M, P>::new() };
                for ((i, j), r) in iproduct!(0..M, 0..P).zip(rows.iter_mut()) {
                    let a: T = self.iter_row(i)
                        .zip(rhs.iter_col(j))
                        .map(|(a, b)| a * b)
                        .sum();
                    *r = MaybeUninit::new(a);
                }
                let rows = unsafe { rows.finish() };
                Matrix::from_array(rows)
            }
        }
    };
    () => {
        impl_mul!(@variant     Matrix<T, M, N>,     Matrix<T, N, P>);
        impl_mul!(@variant &'a Matrix<T, M, N>,     Matrix<T, N, P>);
        impl_mul!(@variant     Matrix<T, M, N>, &'b Matrix<T, N, P>);
        impl_mul!(@variant &'a Matrix<T, M, N>, &'b Matrix<T, N, P>);
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_mul!();

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Zero<Element = T> + LCM + Sub<Output = T> + PartialEq + Clone,
    for<'b> T: Div<&'b T, Output = T>,
    for<'a, 'b> &'a T: Mul<&'b T, Output = T> + Div<&'b T, Output = T>,
{
    pub fn to_echelon(&self) -> Self {
        let mut ech = self.clone();
        let mut r = 0;
        for j in 0..N {
            if let Some(i) = ech.find_nonzero_in_col_after(j, r) {
                if r != i {
                    ech.rows.swap(i, r);
                }
                let a = ech.rows[r][j].clone();
                for i in (r + 1)..M {
                    if ech.rows[i][j] != T::ZERO {
                        let b = ech.rows[i][j].clone();
                        let lcm = b.clone().lcm(a.clone());
                        let c = &lcm / &a;
                        let d = &lcm / &b;
                        for k in 0..N {
                            let s = &d * &ech.rows[i][k] - &c * &ech.rows[r][k];
                            ech.rows[i][k] = s;
                        }
                    }
                }
                r += 1;
            }
        }
        ech
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Zero<Element = T> + One<Element = T> + LCM + Sub<Output = T> + PartialEq + Clone,
    for<'b> T: Div<&'b T, Output = T>,
    for<'a, 'b> &'a T: Mul<&'b T, Output = T> + Div<&'b T, Output = T>,
    [T; N + M]: Sized,
{
    /// Iterator over spanning M length row-vectors in the left-nullspace of the matrix.
    pub fn iter_lnull_span(&self) -> impl Iterator<Item = Vec<T>> {
        let aug = self.augment();
        let ech = aug.to_echelon();
        let subsets: Vec<Vec<T>> = ech
            .rows
            .iter()
            .filter_map(|row| {
                let f = &row[0..N].iter().all(|x| x == &T::ZERO);
                f.then_some(row[N..N + M].into_iter().cloned().collect::<Vec<T>>())
            })
            .collect();
        subsets.into_iter()
    }
}

impl<T, const M: usize, const N: usize> Conjugate for Matrix<T, M, N>
where
    T: Conjugate<Output = T>,
{
    type Output = Matrix<T, N, M>;

    fn conj(&self) -> Matrix<T, N, M> {
        let mut res = self.transpose();
        for a in res.iter_mut_row_major() {
            *a = a.conj();
        }
        res
    }
}

impl<T, const M: usize, const N: usize> Display for Matrix<T, M, N>
where
    T: Display + Clone,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let strings: Vec<String> = self.iter_row_major().map(T::to_string).collect();
        let max_width = strings.iter().map(String::len).max().unwrap();

        macro_rules! write_end {
            ($l:literal, $r:literal) => {
                write!(fmt, $l)?;
                for _ in 0..N {
                    write!(fmt, "{:>w$} ", "", w = max_width)?;
                }
                writeln!(fmt, $r)?;
            };
        }

        write_end!("┌ ", "┐");
        for i in 0..M {
            write!(fmt, "│ ")?;
            for j in 0..N {
                write!(fmt, "{:>w$} ", strings[j + i * N], w = max_width)?;
            }
            writeln!(fmt, "│")?;
        }
        write_end!("└ ", "┘");
        Ok(())
    }
}
