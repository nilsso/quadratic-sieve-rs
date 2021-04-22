use std::convert::From;
use std::{fmt, fmt::Display};

use crate::integers::GCD;

#[derive(Copy, Clone, Debug)]
pub struct RingElement {
    pub value: i32,
    pub base: i32,
    pub inverse: Option<i32>,
    pub order: Option<i32>,
}

fn invertible(n: i32, base: i32) -> bool {
    n.gcd(base) == 1
}

fn inverse(n: i32, base: i32) -> Option<i32> {
    if invertible(n, base) {
        (1..base).filter(|b| (n * b) % base == 1).next()
    } else {
        None
    }
}

fn order(n: i32, base: i32) -> Option<i32> {
    if invertible(n, base) {
        (1..base)
            .filter(|&e| n.pow(e as u32).rem_euclid(base) == 1)
            .next()
    } else {
        None
    }
}

impl RingElement {
    pub fn new(value: i32, base: i32) -> Self {
        let inverse = inverse(value, base);
        let order = order(value, base);
        Self {
            value,
            base,
            inverse,
            order,
        }
    }

    pub fn invertible(&self) -> bool {
        self.inverse.is_some()
    }

    pub fn inverse(&self) -> &Option<i32> {
        &self.inverse
    }

    pub fn order(&self) -> &Option<i32> {
        &self.order
    }
}

impl From<RingElement> for i32 {
    fn from(re: RingElement) -> i32 {
        re.value
    }
}

macro_rules! impl_op {
    ($op:tt) => {
        auto_ops::impl_op_ex!($op |a: &RingElement, b: &RingElement| -> Option<RingElement> {
            if a.base == b.base {
                let value = (a.value $op b.value).rem_euclid(b.base);
                let base = a.base;
                Some(RingElement::new(value, base))
            } else {
                None
            }
        });
    };
    ($($op:tt),*) => {
        $(
            impl_op!($op);
        )*
    };
}
impl_op!(+, -, *);

impl Display for RingElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RingType {
    Group,
    Multiplicative,
}

impl RingType {
    pub fn from_elements(elements: &Vec<RingElement>) -> Self {
        if elements.iter().skip(1).all(|re| re.inverse.is_some()) {
            Self::Multiplicative
        } else {
            Self::Group
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ring {
    base: i32,
    elements: Vec<RingElement>,
    ring_type: RingType,
}

impl Ring {
    pub fn new(base: i32) -> Self {
        let elements = (0..base)
            .map(|value| RingElement::new(value, base))
            .collect();
        let ring_type = RingType::from_elements(&elements);
        Ring {
            base,
            elements,
            ring_type,
        }
    }

    pub fn multiplicative_group(base: i32) -> Self {
        let elements = (0..base)
            .filter(|value| value.gcd(base) == 1)
            .map(|value| RingElement::new(value, base))
            .collect();
        Ring {
            base,
            elements,
            ring_type: RingType::Multiplicative,
        }
    }

    pub fn is_multiplicative_group(&self) -> bool {
        self.elements.len() == self.base as usize
    }

    pub fn order(&self) -> Option<usize> {
        self.is_multiplicative_group().then_some(self.base as usize)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &RingElement> {
        self.elements.iter()
    }
}

impl std::ops::Index<usize> for Ring {
    type Output = RingElement;

    fn index(&self, i: usize) -> &Self::Output {
        &self.elements[i]
    }
}
