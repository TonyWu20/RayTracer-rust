use std::{
    array,
    ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign},
};

use crate::{Scalar, Vector};

use super::{tuple::Tuple};

/// A point in `N`-dimensional space with scalar type `T`.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)]
/// A point in `N`-dimensional space with scalar type `T`.
pub struct Point<T: Scalar, const N: usize>(pub(crate) Tuple<T, N>);

/// In the context of this project, we only deal with 3-dimensional.
/// A point in 3-dimensional space with homogeneous coordinate.
pub type Point3<T> = Point<T, 4>;

impl<T: Scalar, const N: usize> Deref for Point<T, N> {
    type Target = Tuple<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Scalar, const N: usize> DerefMut for Point<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Vector` + `Point` = translated `Point`
impl<T: Scalar, const N: usize> Add<Vector<T, N>> for Point<T, N> {
    type Output = Self;
    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        Self(Tuple(array::from_fn(|i| self[i] + rhs[i])))
    }
}

impl<T: Scalar, const N: usize> AddAssign<Vector<T, N>> for Point<T, N> {
    fn add_assign(&mut self, rhs: Vector<T, N>) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0 .0).zip(rhs.0 .0) {
            *lhs += rhs;
        }
    }
}

/// `Point` B - `Point` A = `Vector` AB
impl<T: Scalar, const N: usize> Sub<Self> for Point<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, rhs: Point<T, N>) -> Self::Output {
        Vector(Tuple(array::from_fn(|i| self[i] - rhs[i])))
    }
}

/// Substract a `Vector` from `Point` = translated `Point`
impl<T: Scalar, const N: usize> Sub<Vector<T, N>> for Point<T, N> {
    type Output = Self;
    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        Self(Tuple(array::from_fn(|i| self[i] - rhs[i])))
    }
}

impl<T: Scalar, const N: usize> SubAssign<Vector<T, N>> for Point<T, N> {
    fn sub_assign(&mut self, rhs: Vector<T, N>) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0 .0).zip(rhs.0 .0) {
            *lhs -= rhs;
        }
    }
}
