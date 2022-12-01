use std::ops::{Deref, DerefMut};

use crate::{Scalar};

use super::tuple::Tuple;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)]
/// An `N`-dimensional vector representing `displacement` with scalar type `T`.
pub struct Vector<T: Scalar, const N: usize>(pub(crate) Tuple<T, N>);

/// In the context of this project, we only deal with 3-dimensional.
/// A vector in 3-dimensional space with homogeneous coordinate.
pub type Vector3<T> = Vector<T, 4>;

impl<T: Scalar, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0 .0
    }
}

impl<T: Scalar, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0 .0
    }
}
