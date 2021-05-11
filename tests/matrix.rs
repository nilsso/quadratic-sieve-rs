use quadratic_sieve::prelude::*;

#[test]
fn test_addition() {
    #[rustfmt::skip]
    let a = Matrix::<i32, 3, 3>::from_array([
        [1, 1, 1],
        [1, 1, 1],
        [1, 1, 1],
    ]);
    #[rustfmt::skip]
    let b = Matrix::<i32, 3, 3>::from_array([
        [2, 2, 2],
        [2, 2, 2],
        [2, 2, 2],
    ]);
    #[rustfmt::skip]
    let res = Matrix::<i32, 3, 3>::from_array([
        [3, 3, 3],
        [3, 3, 3],
        [3, 3, 3],
    ]);
    assert_eq!(a + b, res);
}

#[test]
fn test_subtraction() {
    #[rustfmt::skip]
    let a = Matrix::<i32, 3, 3>::from_array([
        [1, 1, 1],
        [1, 1, 1],
        [1, 1, 1],
    ]);
    #[rustfmt::skip]
    let b = Matrix::<i32, 3, 3>::from_array([
        [2, 2, 2],
        [2, 2, 2],
        [2, 2, 2],
    ]);
    #[rustfmt::skip]
    let res = Matrix::<i32, 3, 3>::from_array([
        [-1, -1, -1],
        [-1, -1, -1],
        [-1, -1, -1],
    ]);
    assert_eq!(a - b, res);
}

#[test]
fn test_multiplication() {
    #[rustfmt::skip]
    let a = Matrix::<i32, 3, 3>::from_array([
        [1, 2, 0],
        [0, 0, 1],
        [3, 0, 1],
    ]);
    #[rustfmt::skip]
    let b = Matrix::<i32, 3, 3>::from_array([
        [2, 0, 3],
        [1, 0, 0],
        [2, 2, 0],
    ]);
    #[rustfmt::skip]
    let res = Matrix::<i32, 3, 3>::from_array([
        [4, 0, 3],
        [2, 2, 0],
        [8, 2, 9],
    ]);
    assert_eq!(a * b, res);
}

#[test]
fn test_multiplication_over_zz5() {
    type T = CongruenceClass<5>;
    const M: usize = 3;
    const N: usize = 3;

    #[rustfmt::skip]
    let a = Matrix::<T, M, N>::from_array(cc_array!(5, [
        [1, 2, 0],
        [0, 0, 1],
        [3, 0, 1],
    ]));
    #[rustfmt::skip]
    let b = Matrix::<T, 3, 3>::from_array(cc_array!(5, [
        [2, 0, 3],
        [1, 0, 0],
        [2, 2, 0],
    ]));
    #[rustfmt::skip]
    let res = Matrix::<T, 3, 3>::from_array(cc_array!(5, [
        [4, 0, 3],
        [2, 2, 0],
        [8, 2, 9], // [3, 2, 4]
    ]));
    assert_eq!(a * b, res);
}

#[test]
fn test_augment() {
    #[rustfmt::skip]
    let a = Matrix::<i32, 4, 2>::from_array([
        [0, 1],
        [2, 3],
        [4, 5],
        [6, 7],
    ]);
    #[rustfmt::skip]
    let res = Matrix::<i32, 4, 6>::from_array([
        [0, 1, 1, 0, 0, 0],
        [2, 3, 0, 1, 0, 0],
        [4, 5, 0, 0, 1, 0],
        [6, 7, 0, 0, 0, 1],
    ]);
    assert_eq!(a.augment(), res)
}

#[test]
fn test_to_echelon() {
    #[rustfmt::skip]
    let a = Matrix::<i32, 3, 4>::from_array([
        [0, 0, 3, 1],
        [2, 2, 1, 1],
        [1, 3, 3, 3],
    ]);
    #[rustfmt::skip]
    let res = Matrix::<i32, 3, 4>::from_array([
        [2, 2, 1, 1],
        [0, 4, 5, 5],
        [0, 0, 3, 1],
    ]);
    assert_eq!(a.to_echelon(), res);
}

#[test]
fn test_to_echelon_zz2() {
    type T = CongruenceClass<2>;
    #[rustfmt::skip]
    let a = Matrix::<T, 4, 4>::from_array(cc_array!(2, [
        [0, 0, 0, 1],
        [0, 0, 0, 0],
        [1, 1, 1, 0],
        [1, 1, 1, 1],
    ]));
    #[rustfmt::skip]
    let res = Matrix::<T, 4, 4>::from_array(cc_array!(2, [
        [1, 1, 1, 0],
        [0, 0, 0, 1],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ]));
    assert_eq!(a.to_echelon(), res);
}

#[test]
fn test_lnull_span_zz2() {
    type T = CongruenceClass<2>;
    #[rustfmt::skip]
    let a = Matrix::<T, 4, 4>::from_array(cc_array!(2, [
        [0, 0, 0, 1],
        [0, 0, 0, 0],
        [1, 1, 1, 0],
        [1, 1, 1, 1],
    ]));
    let mut i = a.iter_lnull_span();
    assert_eq!(i.next().unwrap(), cc_array!(2, [1, 0, 1, 1]));
    assert_eq!(i.next().unwrap(), cc_array!(2, [0, 1, 0, 0]));
}
