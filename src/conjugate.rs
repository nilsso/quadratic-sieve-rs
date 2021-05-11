pub trait Conjugate: Clone {
    type Output;

    fn conj(&self) -> Self::Output;
}

impl Conjugate for u32 {
    type Output = u32;

    fn conj(&self) -> u32 {
        *self
    }
}
