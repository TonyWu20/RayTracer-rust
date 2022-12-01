use std::{
    array,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use bytemuck::{Pod, Zeroable};

use crate::{Float, Point, Scalar};

use super::tuple::{HasX, HasY, HasZ, Tuple};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[repr(transparent)]
/// An `N`-dimensional vector representing `displacement` with scalar type `T`.
pub struct Vector<T: Scalar, const N: usize>(pub(crate) Tuple<T, N>);

// `Zeroable` impls for "Vector" types are sound:
//
// - They are inhabited: structs plus bound `T: Zeroable`.
// - They only consists of `Zeroable` fields, thus zero bit pattern is fine.
unsafe impl<T: Scalar + Zeroable, const N: usize> Zeroable for Vector<T, N> {}

// `Pod` impls for "Vector" types are sound:
//
// - "The type must be inhabited": guaranteed by all being structs and the bound `T: Pod`.
// - "The type must not contain any padding bytes": this is true according to [1].
// - "The type needs to have all fields also be `Pod`": trivially true due to `T: Pod`.
// - "The type must allow any bit pattern": true based on the previous two facts.
// - "The type needs to be `repr(C)` or `repr(transparent)`": trivially true.
//
// [1] https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
unsafe impl<T: Scalar + Pod, const N: usize> Pod for Vector<T, N> {}

impl<T: Scalar, const N: usize> Vector<T, N> {
    /// Returns a vector with all components are zero.
    pub fn zero() -> Self {
        Self::default()
    }
    /// Converts this vector into a point without changing the component values.
    /// The last component is set to be one as a point being a homogeneous
    /// coordinate requires.
    pub fn to_point(self) -> Point<T, N> {
        let mut p = Point(Tuple(self.0 .0));
        p.0 .0[N - 1] = T::one();
        p
    }
    /// Returns the *squared* length of this vector.
    pub fn length2(&self) -> T {
        self.0
             .0
            .iter()
            .map(|&c| c * c)
            .fold(T::zero(), |acc, e| acc + e)
    }
    /// Returns the magnitude (or length) of this vector.
    pub fn magnitude(&self) -> T
    where
        T: Float,
    {
        self.length2().sqrt()
    }
    /// Returns a normalized version of this vector.
    #[must_use = "to normalize in-place, use `Vector::normalize`, not `normalized`"]
    pub fn normalized(mut self) -> Self
    where
        T: Float,
    {
        self.normalize();
        self
    }
    /// Normalizes the vector *in place*.
    pub fn normalize(&mut self)
    where
        T: Float,
    {
        *self = *self / self.magnitude();
    }
    /// Returns a unit vector in x direction.
    pub fn unit_x() -> Self
    where
        Self: HasX<Scalar = T>,
    {
        let mut x = Self::zero();
        *x.x_mut() = T::one();
        x
    }
    /// Returns a unit vector in y direction.
    pub fn unit_y() -> Self
    where
        Self: HasY<Scalar = T>,
    {
        let mut y = Self::zero();
        *y.y_mut() = T::one();
        y
    }
    /// Returns a unit vector in z direction.
    pub fn unit_z() -> Self
    where
        Self: HasZ<Scalar = T>,
    {
        let mut z = Self::zero();
        *z.z_mut() = T::one();
        z
    }
    /// Returns the dot product of this vector and another.
    pub fn dot(&self, rhs: &Vector<T, N>) -> T {
        let mut prod = T::zero();
        for (lhs, rhs) in IntoIterator::into_iter(self.0 .0).zip(rhs.0 .0) {
            prod += lhs * rhs;
        }
        prod
    }
    /// Applies the given function to the `Vector`.
    pub fn map<R: Scalar, F: FnMut(T) -> R>(self, f: F) -> Vector<R, N> {
        Vector(Tuple(self.0 .0.map(f)))
    }
    /// Zip another 'Vector' and then applies the given function.
    pub fn zip_map<U, R, F>(self, other: Vector<U, N>, mut f: F) -> Vector<R, N>
    where
        U: Scalar,
        R: Scalar,
        F: FnMut(T, U) -> R,
    {
        Vector(Tuple(array::from_fn(|i| f(self[i], other[i]))))
    }
}

/// In the context of this project, we only deal with 3-dimensional.
/// A vector in 3-dimensional space with homogeneous coordinate.
pub type Vector3<T> = Vector<T, 4>;

impl<T: Scalar> Vector<T, 4> {
    /// Returns a 3-dimensional vector with homogeneous coordinates.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Tuple([x, y, z, T::zero()]))
    }
    pub fn cross(&self, rhs: &Vector<T, 4>) -> Self {
        Self(Tuple([
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
            T::zero(),
        ]))
    }
}

impl<T: Scalar, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self(Tuple([(); N].map(|_| T::zero())))
    }
}

// impl<T: Scalar, const N: usize> Deref for Vector<T, N> {
//     type Target = [T; N];
//
//     fn deref(&self) -> &Self::Target {
//         &self.0 .0
//     }
// }
//
// impl<T: Scalar, const N: usize> DerefMut for Vector<T, N> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0 .0
//     }
// }

impl<T: Scalar, const N: usize> Add<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        self.zip_map(rhs, |l, r| l + r)
    }
}

impl<T: Scalar, const N: usize> AddAssign<Vector<T, N>> for Vector<T, N> {
    fn add_assign(&mut self, rhs: Vector<T, N>) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0 .0).zip(rhs.0 .0) {
            *lhs += rhs;
        }
    }
}

impl<T: Scalar, const N: usize> Sub<Self> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, |l, r| l - r)
    }
}

impl<T: Scalar, const N: usize> SubAssign<Vector<T, N>> for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Vector<T, N>) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0 .0).zip(rhs.0 .0) {
            *lhs -= rhs;
        }
    }
}

impl<T: Scalar + Neg, const N: usize> Neg for Vector<T, N>
where
    <T as Neg>::Output: Scalar,
{
    type Output = Vector<<T as Neg>::Output, N>;

    fn neg(self) -> Self::Output {
        self.map(|c| -c)
    }
}

/// Scalar multipliation: `vector * scalar`.
impl<T: Scalar, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(|c| c * rhs.clone())
    }
}
// Scalar multiplication: `scalar * vector`. Unfortunately, due to Rust's orphan
// rules, this cannot be implemented generically. So we just implement it for
// core primitive types.
macro_rules! impl_scalar_mul {
    ($($ty:ident),*) => {
        $(
            impl<const N: usize> Mul<Vector<$ty, N>> for $ty {
                type Output = Vector<$ty, N>;
                fn mul(self, rhs: Vector<$ty, N>) -> Self::Output {
                    rhs * self
                }
            }
        )*
    };
}

impl_scalar_mul!(f32, f64, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

/// Scalar multipliation: `vector *= scalar`.
impl<T: Scalar, const N: usize> MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        for c in &mut self.0 .0 {
            *c *= rhs.clone();
        }
    }
}

/// Scalar division: `vector / scalar`.
impl<T: Scalar, const N: usize> Div<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, rhs: T) -> Self::Output {
        self.map(|c| c / rhs.clone())
    }
}

/// Scalar division: `vector /= scalar`.
impl<T: Scalar, const N: usize> DivAssign<T> for Vector<T, N> {
    fn div_assign(&mut self, rhs: T) {
        for c in &mut self.0 .0 {
            *c /= rhs;
        }
    }
}
impl<T: Scalar, const N: usize> std::iter::Sum<Self> for Vector<T, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

// Implementation of direct access the `[T;N]` inside the wrapped `Tuple`.
impl<T: Scalar, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0 .0[index]
    }
}
// Implementation of direct access the `[T;N]` inside the wrapped `Tuple`.
impl<T: Scalar, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0 .0[index]
    }
}
// Implementation of construction from `[T;N]` with `From`.
impl<T: Scalar, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(src: [T; N]) -> Self {
        Self(Tuple::from(src))
    }
}
// Implementation of construction from `Vector<T,N>` to `[T;N]` with `From`.
impl<T: Scalar, const N: usize> From<Vector<T, N>> for [T; N] {
    fn from(src: Vector<T, N>) -> Self {
        src.0 .0
    }
}
// Construct a homogeneous coordinate `Vector<T,4>` (alias `Point3<T>`)
// from an array of size 3.
impl<T: Scalar> From<[T; 3]> for Vector<T, 4> {
    fn from(src: [T; 3]) -> Self {
        let [x, y, z] = src;
        Self::new(x, y, z)
    }
}
// Construct an array with a size of 3 (`[x,y,z]`) from `Vector3`
impl<T: Scalar> From<Vector<T, 4>> for [T; 3] {
    fn from(src: Vector<T, 4>) -> Self {
        [src.x, src.y, src.z]
    }
}
// Implementation of `AsRef` for `Vector` to borrow the inner array.
impl<T: Scalar, const N: usize> AsRef<[T; N]> for Vector<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.0 .0
    }
}
// Implementation of `AsMut` for `Vector` to mutably borrow the inner array.
impl<T: Scalar, const N: usize> AsMut<[T; N]> for Vector<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.0 .0
    }
}
