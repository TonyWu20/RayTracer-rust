//! Implementation of `Tuple`, the basic struct to represent a point or a vector.
//! Implement access by name (`x`, `y`, `z` and `w`) referring to `lina`.
//! Due to the context of this lib, we only deal with 3D homogeneous points or vectors,
//! therefore, we just need to implement the traits for `Tuple<T,4>`.
use bytemuck::{Pod, Zeroable};
use std::{
    array,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
        SubAssign,
    },
};

use crate::{Point, Scalar, Vector};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Tuple<T: Scalar, const N: usize>(pub(crate) [T; N]);

unsafe impl<T: Scalar + Zeroable, const N: usize> Zeroable for Tuple<T, N> {}
unsafe impl<T: Scalar + Pod, const N: usize> Pod for Tuple<T, N> {}

/// Helper struct giving access to the individual components of a 4D
/// tuple.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct View4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

// `Zeroable` impls for "view" types are sound:
//
// - They are inhabited: structs plus bound `T: Zeroable`.
// - They only consists of `Zeroable` fields, thus zero bit pattern is fine.
unsafe impl<T: Zeroable> Zeroable for View4<T> {}

// `Pod` impls for "view" types are sound:
//
// - "The type must be inhabited": guaranteed by all being structs and the bound `T: Pod`.
// - "The type must not contain any padding bytes": this is true according to [1].
// - "The type needs to have all fields also be `Pod`": trivially true due to `T: Pod`.
// - "The type must allow any bit pattern": true based on the previous two facts.
// - "The type needs to be `repr(C)` or `repr(transparent)`": trivially true.
//
// [1] https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
unsafe impl<T: Pod> Pod for View4<T> {}

// `Deref` and `DerefMut` impls to enable `.x` like field access.
// Due to the context of this lib, we only deal with 3D homogeneous points or vectors,
// therefore, we just need to implement the traits for `Tuple<T,4>`.
macro_rules! impl_view_deref {
    ($ty:ident, $n:expr, $view_ty:ident) => {
        impl<T: Scalar> Deref for $ty<T, $n> {
            type Target = $view_ty<T>;
            fn deref(&self) -> &Self::Target {
                bytemuck::cast_ref(self)
            }
        }
        impl<T: Scalar> DerefMut for $ty<T, $n> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                bytemuck::cast_mut(self)
            }
        }
    };
}
impl_view_deref!(Vector, 4, View4);
impl_view_deref!(Point, 4, View4);

impl<T: Scalar> Deref for Tuple<T, 4> {
    type Target = View4<T>;

    fn deref(&self) -> &Self::Target {
        bytemuck::cast_ref(self)
    }
}

impl<T: Scalar> DerefMut for Tuple<T, 4> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        bytemuck::cast_mut(self)
    }
}

/// Vectors or points that have an `x` component.
pub trait HasX {
    type Scalar;
    fn x(&self) -> &Self::Scalar;
    fn x_mut(&mut self) -> &mut Self::Scalar;
}

/// Vectors or points that have an `y` component.
pub trait HasY {
    type Scalar;
    fn y(&self) -> &Self::Scalar;
    fn y_mut(&mut self) -> &mut Self::Scalar;
}

/// Vectors or points that have an `z` component.
pub trait HasZ {
    type Scalar;
    fn z(&self) -> &Self::Scalar;
    fn z_mut(&mut self) -> &mut Self::Scalar;
}

/// Vectors or points that have an `w` component.
pub trait HasW {
    type Scalar;
    fn w(&self) -> &Self::Scalar;
    fn w_mut(&mut self) -> &mut Self::Scalar;
}
macro_rules! impl_has_axis {
    ($ty:ident, $d:expr, $trait:ident, $i:expr, $axis:ident, $axis_mut:ident) => {
        impl<T: Scalar> $trait for $ty<T, $d> {
            type Scalar = T;
            fn $axis(&self) -> &Self::Scalar {
                &self[$i]
            }
            fn $axis_mut(&mut self) -> &mut Self::Scalar {
                &mut self[$i]
            }
        }
    };
}

impl_has_axis!(Tuple, 4, HasX, 0, x, x_mut);
impl_has_axis!(Tuple, 4, HasY, 1, y, y_mut);
impl_has_axis!(Tuple, 4, HasZ, 2, z, z_mut);
impl_has_axis!(Tuple, 4, HasW, 3, w, w_mut);
impl_has_axis!(Point, 4, HasX, 0, x, x_mut);
impl_has_axis!(Point, 4, HasY, 1, y, y_mut);
impl_has_axis!(Point, 4, HasZ, 2, z, z_mut);
impl_has_axis!(Point, 4, HasW, 3, w, w_mut);
impl_has_axis!(Vector, 4, HasX, 0, x, x_mut);
impl_has_axis!(Vector, 4, HasY, 1, y, y_mut);
impl_has_axis!(Vector, 4, HasZ, 2, z, z_mut);
impl_has_axis!(Vector, 4, HasW, 3, w, w_mut);

impl<T: Scalar, const N: usize> Index<usize> for Tuple<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Scalar, const N: usize> IndexMut<usize> for Tuple<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Scalar, const N: usize> From<[T; N]> for Tuple<T, N> {
    fn from(src: [T; N]) -> Self {
        Self(src)
    }
}

impl<T: Scalar, const N: usize> From<Tuple<T, N>> for [T; N] {
    fn from(src: Tuple<T, N>) -> Self {
        src.0
    }
}

impl<T: Scalar, const N: usize> AsRef<[T; N]> for Tuple<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.0
    }
}

impl<T: Scalar, const N: usize> AsMut<[T; N]> for Tuple<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }
}

impl<T: Scalar, const N: usize> Add for Tuple<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl<T: Scalar, const N: usize> AddAssign for Tuple<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0).zip(rhs.0) {
            *lhs += rhs
        }
    }
}

impl<T: Scalar, const N: usize> Sub for Tuple<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl<T: Scalar, const N: usize> SubAssign for Tuple<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0).zip(rhs.0) {
            *lhs -= rhs
        }
    }
}
impl<T: Scalar + Neg, const N: usize> Neg for Tuple<T, N>
where
    <T as Neg>::Output: Scalar,
{
    type Output = Tuple<<T as Neg>::Output, N>;

    fn neg(self) -> Self::Output {
        let Self(t) = self;
        Tuple(t.map(|c| -c))
    }
}
/// Scalar multipliation: `tuple` * scalar`.
impl<T: Scalar, const N: usize> Mul<T> for Tuple<T, N> {
    type Output = Tuple<T, N>;
    fn mul(self, rhs: T) -> Self::Output {
        let Self(t) = self;
        Tuple(t.map(|c| c * rhs))
    }
}
/// Scalar multipliation: `Tuple *= scalar`.
impl<T: Scalar, const N: usize> MulAssign<T> for Tuple<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        let Self(t) = self;
        for c in t {
            *c *= rhs;
        }
    }
}
/// Scalar division: `Tuple / scalar`.
impl<T: Scalar, const N: usize> Div<T> for Tuple<T, N> {
    type Output = Tuple<T, N>;

    fn div(self, rhs: T) -> Self::Output {
        Tuple(self.0.map(|c| c / rhs))
    }
}
/// Scalar division: `Tuple /= scalar`.
impl<T: Scalar, const N: usize> DivAssign<T> for Tuple<T, N> {
    fn div_assign(&mut self, rhs: T) {
        for c in &mut self.0 {
            *c /= rhs;
        }
    }
}
