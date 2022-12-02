use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

use bytemuck::{Pod, Zeroable};

use crate::{Scalar, Vector};

use super::tuple::Tuple;

/// A point in `N`-dimensional space with scalar type `T`.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[repr(transparent)]
/// A point in `N`-dimensional space with scalar type `T`.
pub struct Point<T: Scalar, const N: usize>(pub(crate) Tuple<T, N>);

// `Zeroable` impls for "Point" types are sound:
//
// - They are inhabited: structs plus bound `T: Zeroable`.
// - They only consists of `Zeroable` fields, thus zero bit pattern is fine.
unsafe impl<T: Scalar + Zeroable, const N: usize> Zeroable for Point<T, N> {}

// `Pod` impls for "Point" types are sound:
//
// - "The type must be inhabited": guaranteed by all being structs and the bound `T: Pod`.
// - "The type must not contain any padding bytes": this is true according to [1].
// - "The type needs to have all fields also be `Pod`": trivially true due to `T: Pod`.
// - "The type must allow any bit pattern": true based on the previous two facts.
// - "The type needs to be `repr(C)` or `repr(transparent)`": trivially true.
//
// [1] https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
unsafe impl<T: Scalar + Pod, const N: usize> Pod for Point<T, N> {}

impl<T: Scalar, const N: usize> Point<T, N> {
    pub fn origin() -> Self {
        Self::default()
    }
    /// Converts this point into a vector by subtracting with the origin.
    /// This will blindly change the type.
    pub fn to_vec(self) -> Vector<T, N> {
        self - Self::origin()
    }
    pub fn centroid(points: impl IntoIterator<Item = Self>) -> Option<Self> {
        let mut it = points.into_iter();
        let mut total_displacement = it.next()?.to_vec();
        let mut count = T::one();
        for p in it {
            total_displacement += p.to_vec();
            count += T::one();
        }
        Some((total_displacement / count).to_point())
    }
}

/// In the context of this project, we only deal with 3-dimensional.
/// A point in 3-dimensional space with homogeneous coordinate.
pub type Point3<T> = Point<T, 4>;

impl<T: Scalar> Point<T, 4> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Tuple([x, y, z, T::one()]))
    }
}

impl<T: Scalar, const N: usize> Default for Point<T, N> {
    fn default() -> Self {
        let mut data = [(); N].map(|_| T::zero());
        data[N - 1] = T::one();
        Self(Tuple(data))
    }
}

/// `Vector` + `Point` = translated `Point`
impl<T: Scalar, const N: usize> Add<Vector<T, N>> for Point<T, N> {
    type Output = Self;
    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        // Semantically equivalent to `self.0 + rhs.0`
        // But easier to understand.
        let Self(tp) = self;
        let Vector::<T, N>(tv) = rhs;
        Self(tp + tv)
    }
}

impl<T: Scalar, const N: usize> AddAssign<Vector<T, N>> for Point<T, N> {
    fn add_assign(&mut self, rhs: Vector<T, N>) {
        let Self(tp) = self;
        let Vector::<T, N>(tv) = rhs;
        *tp += tv;
    }
}

/// `Point` B - `Point` A = `Vector` AB
impl<T: Scalar, const N: usize> Sub<Self> for Point<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, rhs: Point<T, N>) -> Self::Output {
        let Self(tp1) = self;
        let Self(tp2) = rhs;
        Vector(tp1 - tp2)
    }
}

/**
Substract a `Vector` from `Point` = translated `Point`
*/
impl<T: Scalar, const N: usize> Sub<Vector<T, N>> for Point<T, N> {
    type Output = Self;
    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        let Self(tp) = self;
        let Vector::<T, N>(tv) = rhs;
        Self(tp - tv)
    }
}

impl<T: Scalar, const N: usize> SubAssign<Vector<T, N>> for Point<T, N> {
    fn sub_assign(&mut self, rhs: Vector<T, N>) {
        let Self(tp) = self;
        let Vector::<T, N>(tv) = rhs;
        *tp -= tv;
    }
}

// Implementation of direct access the `[T;N]` inside the wrapped `Tuple`.
impl<T: Scalar, const N: usize> Index<usize> for Point<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0 .0[index]
    }
}
// Implementation of direct access the `[T;N]` inside the wrapped `Tuple`.
impl<T: Scalar, const N: usize> IndexMut<usize> for Point<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0 .0[index]
    }
}
// Implementation of construction from `[T;N]` with `From`.
impl<T: Scalar, const N: usize> From<[T; N]> for Point<T, N> {
    fn from(src: [T; N]) -> Self {
        Self(Tuple::from(src))
    }
}
// Implementation of construction from `Point<T,N>` to `[T;N]` with `From`.
impl<T: Scalar, const N: usize> From<Point<T, N>> for [T; N] {
    fn from(src: Point<T, N>) -> Self {
        src.0 .0
    }
}
// Construct a homogeneous coordinate `Point<T,4>` (alias `Point3<T>`)
// from an array of size 3.
impl<T: Scalar> From<[T; 3]> for Point<T, 4> {
    fn from(src: [T; 3]) -> Self {
        let [x, y, z] = src;
        Self::new(x, y, z)
    }
}
// Construct an array with a size of 3 (`[x,y,z]`) from `Point3`
impl<T: Scalar> From<Point<T, 4>> for [T; 3] {
    fn from(src: Point<T, 4>) -> Self {
        [src.x, src.y, src.z]
    }
}
// Implementation of `AsRef` for `Point` to borrow the inner array.
impl<T: Scalar, const N: usize> AsRef<[T; N]> for Point<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.0 .0
    }
}
// Implementation of `AsMut` for `Point` to mutably borrow the inner array.
impl<T: Scalar, const N: usize> AsMut<[T; N]> for Point<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.0 .0
    }
}
