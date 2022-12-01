#![allow(dead_code)]

use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use bytemuck::Pod;
use num_traits::Num;
pub const EPSILON: f64 = 0.0001;

pub mod docs;
pub mod features;

pub use features::linalg::{point::Point, vector::Vector};

/// A scalar type in the context of this library, following `lina`.
/// This is implemented for at least these types:
///
/// - Floats: `f32` and `f64`
/// - Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
/// - Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
pub trait Scalar:
    Num + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign + DivAssign + Pod
{
}

impl<T> Scalar for T where
    T: Num + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign + DivAssign + Pod
{
}

/// A floating point scalar.
///
/// This is similar to [`Scalar`] as it defines coarse requirements for using
/// functions of this library. It is used whenever `Scalar` is not sufficient,
/// which is basically whenever a function does not make sense for integers.
/// This trait is implemented for at least `f32` and `f64`.
pub trait Float: Scalar + num_traits::Float + num_traits::FloatConst {
    fn two() -> Self {
        Self::one() + Self::one()
    }
    fn three() -> Self {
        Self::one() + Self::one() + Self::one()
    }
    fn four() -> Self {
        Self::one() + Self::one() + Self::one() + Self::one()
    }
}

impl<T> Float for T where T: Scalar + num_traits::Float + num_traits::FloatConst {}
