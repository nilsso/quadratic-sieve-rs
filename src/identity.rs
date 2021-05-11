pub trait Zero {
    type Element;

    const ZERO: Self::Element;
}

pub trait One {
    type Element;

    const ONE: Self::Element;
}

macro_rules! impl_integer_identities {
    ($($T:ty),*) => {
        $(
            impl Zero for $T {
                type Element = $T;

                const ZERO: $T = 0;
            }

            impl One for $T {
                type Element = $T;

                const ONE: $T = 1;
            }
        )*
    };
}

impl_integer_identities!(u8, u16, u32, u64, i8, i16, i32, i64);

impl Zero for bool {
    type Element = bool;

    const ZERO: bool = false;
}

impl One for bool {
    type Element = bool;

    const ONE: bool = true;
}
