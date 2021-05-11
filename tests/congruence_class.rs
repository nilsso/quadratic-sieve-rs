use quadratic_sieve::prelude::*;

#[test]
fn test_constructor_reduces() {
    assert_eq!(cc!(10, 5), cc!(0, 5));
    assert_eq!(cc!(11, 10), cc!(1, 10));
    assert_eq!(cc!(99, 100), cc!(99, 100));
}

#[test]
fn test_addition() {
    let a = CongruenceClass::<7>::new(5);
    let b = CongruenceClass::<7>::new(6);
    let res = CongruenceClass::<7>::new(4); // 11 ≡ 4 mod 7
    assert_eq!(a + b, res);
}

#[test]
fn test_multiplication() {
    let a = CongruenceClass::<7>::new(5);
    let b = CongruenceClass::<7>::new(6);
    let res = CongruenceClass::<7>::new(2); // 30 ≡ 2 mod 7
    assert_eq!(a * b, res);
}

// TODO: Add compilation failure checking crate
// https://crates.io/crates/compiletest-rs
/*
 *#[test]
 *fn test_cannot_add() {
 *    let a = CongruenceClass::<5>::new(6);
 *    let b = CongruenceClass::<6>::new(7);
 *    a + b;
 *}
 */
